pub trait Markup {
    fn markup(&self) -> Option<String>;
}

impl Markup for todo_txt::task::Note {
    fn markup(&self) -> Option<String> {
        let content = match self.content() {
            Some(content) => content,
            None => return None,
        };

        let parser = pulldown_cmark::Parser::new(&content);

        let mut markup = String::from("<markup>");

        let headers = vec![
            "xx-large", "x-large", "large", "medium", "small", "x-small", "xx-small",
        ];

        for event in parser {
            use pulldown_cmark::Event;
            use pulldown_cmark::Tag;

            match event {
                Event::Start(Tag::Heading(level)) => markup.push_str(&format!(
                    "<span font_size='{}'><u>",
                    headers[level as usize]
                )),
                Event::End(Tag::Heading(_)) => markup.push_str("</u></span>\n\n"),

                Event::Start(Tag::Paragraph) => markup.push_str("<span>"),
                Event::End(Tag::Paragraph) => markup.push_str("</span>\n"),

                Event::Start(Tag::CodeBlock(_)) => markup.push_str("<tt>"),
                Event::End(Tag::CodeBlock(_)) => markup.push_str("</tt>"),

                Event::Start(Tag::Emphasis) => markup.push_str("<i>"),
                Event::End(Tag::Emphasis) => markup.push_str("</i>"),

                Event::Start(Tag::Strong) => markup.push_str("<b>"),
                Event::End(Tag::Strong) => markup.push_str("</b>"),

                Event::Start(Tag::Item) => markup.push_str("· "),
                Event::End(Tag::Item) | Event::SoftBreak => markup.push('\n'),

                Event::Start(Tag::Link(_, link, title)) => {
                    markup.push_str(&format!("<a href='{}' title='{}'>", link, title))
                }
                Event::End(Tag::Link(_, _, _)) => markup.push_str("</a>"),

                Event::Text(t) => markup.push_str(&t.replace("&", "&amp;")),

                _ => (),
            }
        }
        markup.push_str("</markup>");

        Some(markup)
    }
}
