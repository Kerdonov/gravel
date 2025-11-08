mod block;
mod inline;

use block::parse_blocks;

use crate::ast::Document;

pub fn parse(s: &str) -> Document {
    Document {
        blocks: parse_blocks(s),
    }
}

#[cfg(test)]
mod test {
    use crate::ast::*;
    use crate::parser::parse;

    #[test]
    fn only_paragraph() {
        let md = "testing paragraph";

        let doc = parse(md);
        assert_eq!(
            doc,
            Document {
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "testing paragraph".to_string()
                )])]
            }
        );
    }

    #[test]
    fn different_headers() {
        let md = "# Header 1\n## Header 2";

        let doc = parse(md);

        assert_eq!(
            doc,
            Document {
                blocks: vec![
                    Block::Heading {
                        level: 1,
                        content: vec![Inline::Text("Header 1".to_string())]
                    },
                    Block::Heading {
                        level: 2,
                        content: vec![Inline::Text("Header 2".to_string())]
                    },
                ]
            }
        );
    }

    #[test]
    fn inline_bold_and_italics() {
        let md = "some *bold* and _italic_ text";

        let doc = parse(md);

        assert_eq!(
            doc,
            Document {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Text("some ".to_string()),
                    Inline::Bold(vec![Inline::Text("bold".to_string())]),
                    Inline::Text(" and ".to_string()),
                    Inline::Italic(vec![Inline::Text("italic".to_string())]),
                    Inline::Text(" text".to_string()),
                ])]
            }
        );
    }

    #[test]
    fn inline_code() {
        let md = "run command `sudo rm -rf /`";

        let doc = parse(md);

        assert_eq!(
            doc,
            Document {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Text("run command ".to_string()),
                    Inline::Code("sudo rm -rf /".to_string()),
                ])]
            }
        );
    }

    #[test]
    fn bold_header() {
        let md = "# Header is *bold*";

        let doc = parse(md);

        assert_eq!(
            doc,
            Document {
                blocks: vec![Block::Heading {
                    level: 1,
                    content: vec![
                        Inline::Text("Header is ".to_string()),
                        Inline::Bold(vec![Inline::Text("bold".to_string())])
                    ]
                }]
            }
        );
    }

    #[test]
    fn anonymous_code_block() {
        let md = "```\necho hello\n```";

        let doc = parse(md);

        assert_eq!(
            doc,
            Document {
                blocks: vec![Block::Code {
                    language: None,
                    content: "echo hello\n".to_string()
                }]
            }
        );
    }

    #[test]
    fn rust_code_block() {
        let md = "```rust\nfn main() {\n\tprintln!(\"Hello world!\");\n}\n```";

        let doc = parse(md);

        assert_eq!(
            doc,
            Document {
                blocks: vec![Block::Code {
                    language: Some("rust".to_string()),
                    content: "fn main() {\n\tprintln!(\"Hello world!\");\n}\n".to_string()
                }]
            }
        );
    }
}
