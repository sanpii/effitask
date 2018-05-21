#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Note {
    None,
    Short(String),
    Long { filename: String, content: String },
}

impl Note {
    pub fn from_file(filename: &str) -> Self {
        use std::io::Read;

        if filename.is_empty() {
            return Note::None;
        }

        let note_file = match Self::note_file(filename) {
            Ok(note_file) => note_file,
            Err(err) => {
                error!("{}", err);
                return Note::Short(filename.to_string());
            }
        };

        let file = match ::std::fs::File::open(note_file.clone()) {
            Ok(file) => file,
            Err(_) => {
                error!("Unable to open {:?}", note_file);
                return Note::Short(filename.to_string());
            }
        };

        let mut buffer = ::std::io::BufReader::new(file);
        let mut content = String::new();

        match buffer.read_to_string(&mut content) {
            Ok(_) => (),
            Err(_) => {
                error!("Unable to read {:?}", note_file);
                return Note::Short(filename.to_string());
            }
        };

        Note::Long {
            filename: filename.to_string(),
            content,
        }
    }

    pub fn content(&self) -> Option<String> {
        match *self {
            Note::None => None,
            Note::Short(ref content) | Note::Long { ref content, .. } => Some(content.clone()),
        }
    }

    pub fn markup(&self) -> Option<String> {
        let content = match self.content() {
            Some(content) => content,
            None => return None,
        };

        let parser = ::pulldown_cmark::Parser::new(&content);

        let mut markup = String::from("<markup>");

        let headers = vec![
            "xx-large", "x-large", "large", "medium", "small", "x-small", "xx-small"
        ];

        for event in parser {
            use pulldown_cmark::Event;
            use pulldown_cmark::Tag;

            match event {
                Event::Start(Tag::Header(level)) => markup.push_str(&format!(
                    "<span font_size='{}'><u>",
                    headers[level as usize]
                )),
                Event::End(Tag::Header(_)) => markup.push_str("</u></span>\n\n"),

                Event::Start(Tag::Paragraph) => markup.push_str("<span>"),
                Event::End(Tag::Paragraph) => markup.push_str("</span>\n"),

                Event::Start(Tag::Code) | Event::Start(Tag::CodeBlock(_)) => {
                    markup.push_str("<tt>")
                }
                Event::End(Tag::Code) | Event::End(Tag::CodeBlock(_)) => markup.push_str("</tt>"),

                Event::Start(Tag::Emphasis) => markup.push_str("<i>"),
                Event::End(Tag::Emphasis) => markup.push_str("</i>"),

                Event::Start(Tag::Strong) => markup.push_str("<b>"),
                Event::End(Tag::Strong) => markup.push_str("</b>"),

                Event::Start(Tag::Item) => markup.push_str("· "),
                Event::End(Tag::Item) | Event::SoftBreak => markup.push_str("\n"),

                Event::Start(Tag::Link(link, title)) => {
                    markup.push_str(&format!("<a href='{}' title='{}'>", link, title))
                }
                Event::End(Tag::Link(_, _)) => markup.push_str("</a>"),

                Event::Text(t) => markup.push_str(&t.replace("&", "&amp;")),

                _ => (),
            }
        }
        markup.push_str("</markup>");

        Some(markup)
    }

    pub fn write(&self) -> Result<Self, String> {
        let mut note = self.clone();

        if self == &Note::None {
            return Ok(note);
        }

        if let Note::Short(ref content) = *self {
            note = Note::Long {
                filename: Self::new_filename(),
                content: content.clone(),
            }
        }

        if let Note::Long {
            ref filename,
            ref content,
        } = note
        {
            if content.is_empty() {
                match ::std::fs::remove_file(Self::note_file(filename)?) {
                    Ok(_) => (),
                    Err(err) => error!("Unable to delete note: {}", err),
                };

                return Ok(Note::None);
            }
        }

        if let Note::Long {
            ref filename,
            ref content,
        } = note
        {
            use std::io::Write;

            let note_file = Self::note_file(filename)?;

            let mut f = match ::std::fs::File::create(note_file) {
                Ok(f) => f,
                Err(err) => return Err(format!("{}", err)),
            };

            match f.write(format!("{}", content).as_bytes()) {
                Ok(_) => (),
                Err(err) => return Err(format!("{}", err)),
            };
        }

        Ok(note)
    }

    fn new_filename() -> String {
        let ext = match ::std::env::var("TODO_NOTE_EXT") {
            Ok(ext) => ext,
            Err(_) => ".txt".to_owned(),
        };

        let name = Self::new_note_id();

        format!("{}{}", name, ext)
    }

    fn new_note_id() -> String {
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        ::rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(3)
            .collect()
    }

    fn note_file(filename: &str) -> Result<String, String> {
        let todo_dir = match ::std::env::var("TODO_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(_) => return Err("Launch this program via todo.sh".to_owned()),
        };

        let note_dir = match ::std::env::var("TODO_NOTES_DIR") {
            Ok(note_dir) => note_dir,
            Err(_) => format!("{}/notes", todo_dir),
        };

        Ok(format!("{}/{}", note_dir, filename))
    }
}

impl ::std::fmt::Display for Note {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let tag = match ::std::env::var("TODO_NOTE_TAG") {
            Ok(tag) => tag,
            Err(_) => "note".to_owned(),
        };

        let tag = match *self {
            Note::None => String::new(),
            Note::Short(ref content) => format!("{}:{}", tag, content),
            Note::Long { ref filename, .. } => format!("{}:{}", tag, filename),
        };

        f.write_str(tag.as_str())
    }
}
