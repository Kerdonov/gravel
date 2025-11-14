//! A trait + implementations for generating HTML.

use crate::ast::{Block, Document, Inline};

pub trait ToHtml {
    fn to_html(self) -> String;
}

impl ToHtml for Document {
    fn to_html(self) -> String {
        format!(
            "<!doctype html><html lang=en><head></head><body>{}</body></html>",
            self.blocks.to_html()
        )
    }
}

impl ToHtml for Block {
    fn to_html(self) -> String {
        match self {
            Self::Paragraph(content) => format!("<p>{}</p>", content.to_html()),
            Self::Heading { level, content } => {
                format!("<h{}>{}</h{}>", level, content.to_html(), level)
            }
            Self::Code {
                language: _,
                content,
            } => {
                format!("<pre><code>{content}</code></pre>")
            }
            _ => todo!(),
        }
    }
}

impl ToHtml for Inline {
    fn to_html(self) -> String {
        match self {
            Self::Text(s) => s,
            Self::Bold(content) => format!("<b>{}</b>", content.to_html()),
            Self::Italic(content) => format!("<i>{}</i>", content.to_html()),
            Self::Code(s) => format!("<code>{s}</code>"),
            Self::Link { text, href } => format!("<a href=\"{}\">{}</a>", href, text.to_html()),
        }
    }
}

impl<T> ToHtml for Vec<T>
where
    T: ToHtml,
{
    fn to_html(self) -> String {
        let mut rendered = String::new();
        for i in self {
            rendered.push_str(&i.to_html());
        }
        rendered
    }
}

// --------------------
// TESTS
// --------------------

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn single_header() {
        let ast = Document {
            blocks: vec![Block::Heading {
                level: 1,
                content: vec![Inline::Text("Heading 1".to_string())],
            }],
        };

        let html = ast.to_html();

        assert_eq!(
            html,
            "<!doctype html><html lang=en><head></head><body><h1>Heading 1</h1></body></html>"
        );
    }

    #[test]
    fn inline_bold_header() {
        let ast = Document {
            blocks: vec![Block::Heading {
                level: 1,
                content: vec![
                    Inline::Bold(vec![Inline::Text("Bold".to_string())]),
                    Inline::Text(" heading 1".to_string()),
                ],
            }],
        };

        let html = ast.to_html();

        assert_eq!(
            html,
            "<!doctype html><html lang=en><head></head><body><h1><b>Bold</b> heading 1</h1></body></html>"
        );
    }

    #[test]
    fn headings_and_paragraph_nested_code() {
        let ast = Document {
            blocks: vec![
                Block::Heading {
                    level: 1,
                    content: vec![
                        Inline::Bold(vec![Inline::Text("Bold".to_string())]),
                        Inline::Text(" heading 1".to_string()),
                    ],
                },
                Block::Heading {
                    level: 2,
                    content: vec![Inline::Text("Heading 2".to_string())],
                },
                Block::Paragraph(vec![
                    Inline::Text("run ".to_string()),
                    Inline::Code("sudo rm -rf /".to_string()),
                    Inline::Text(" on your computer".to_string()),
                ]),
            ],
        };

        let html = ast.to_html();

        assert_eq!(
            html,
            "<!doctype html><html lang=en><head></head><body><h1><b>Bold</b> heading 1</h1><h2>Heading 2</h2><p>run <code>sudo rm -rf /</code> on your computer</p></body></html>"
        );
    }
}

#[cfg(test)]
mod convert_md_to_html_test {
    use crate::parser::parse;
    use crate::to_html::ToHtml;

    #[test]
    fn single_header() {
        let md = "# Header 1";
        let ast = match parse(md) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };
        let html = ast.to_html();

        assert_eq!(
            html,
            "<!doctype html><html lang=en><head></head><body><h1>Header 1</h1></body></html>"
        );
    }

    #[test]
    fn single_header_wrong_format() {
        let md = "#Whoops";
        let ast = parse(md);

        assert!(ast.is_err());
    }

    #[test]
    fn nested_bold_headers_and_nested_code_paragraph() {
        let md = "# *Bold* header 1\n## Header 2\nrun `sudo rm -rf /` on your computer";
        let ast = match parse(md) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };
        let html = ast.to_html();

        assert_eq!(
            html,
            "<!doctype html><html lang=en><head></head><body><h1><b>Bold</b> header 1</h1><h2>Header 2</h2><p>run <code>sudo rm -rf /</code> on your computer</p></body></html>"
        );
    }
}

#[cfg(test)]
mod parse_real_md {
    use std::fs;

    use crate::parser::parse;

    #[test]
    fn go() {
        let file = "./test.md";
        let md = fs::read_to_string(file).expect("reading ./test.md failed");

        let _ast = match parse(&md).map_err(|e| e.set_file(file.into())) {
            Ok(a) => a,
            Err(e) => panic!("{}", e),
        };
    }
}
