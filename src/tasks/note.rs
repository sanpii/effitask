#[derive(Clone, Debug)]
pub enum Note {
    None,
    Short(String),
    Long {
        filename: String,
        content: String,
    },
}

impl Note
{
    pub fn from_file(filename: &String) -> Self
    {
        use std::io::Read;

        if filename.is_empty() {
            return Note::None;
        }

        let todo_dir = match ::std::env::var("TODO_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(_) => {
                error!("Launch this program via todo.sh");
                return Note::Short(filename.clone());
            },
        };

        let note_dir = match ::std::env::var("TODO_NOTES_DIR") {
            Ok(todo_dir) => todo_dir,
            Err(_) => format!("{}/notes", todo_dir),
        };

        let note_file = format!("{}/{}", note_dir, filename);

        let file = match ::std::fs::File::open(note_file.clone()) {
            Ok(file) => file,
            Err(_) => {
                error!("Unable to open {:?}", note_file);
                return Note::Short(filename.clone());
            },
        };

        let mut buffer = ::std::io::BufReader::new(file);
        let mut content = String::new();

        match buffer.read_to_string(&mut content) {
            Ok(_) => (),
            Err(_) => {
                error!("Unable to read {:?}", note_file);
                return Note::Short(filename.clone());
            },
        };

        Note::Long {
            filename: note_file,
            content: content,
        }
    }

    pub fn content(&self) -> Option<String>
    {
        match self {
            &Note::None => None,
            &Note::Short(ref content) => Some(content.clone()),
            &Note::Long { filename: _, ref content } => Some(content.clone()),
        }
    }
}

impl ::std::fmt::Display for Note
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
    {
        let tag = match self {
            &Note::None => String::new(),
            &Note::Short(ref content) => format!("note:{}", content),
            &Note::Long { ref filename, content: _ } => format!("note:{}", filename),
        };

        f.write_str(tag.as_str())
    }
}
