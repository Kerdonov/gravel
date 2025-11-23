use crate::{MdParseError, ast::Inline};

pub fn parse_inlines(input: &str) -> Result<Vec<Inline>, MdParseError> {
    let mut inlines = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '*' => {
                let inner = collect_until(&mut chars, '*')?;
                inlines.push(Inline::Bold(parse_inlines(&inner)?));
            }
            '_' => {
                let inner = collect_until(&mut chars, '_')?;
                inlines.push(Inline::Italic(parse_inlines(&inner)?));
            }
            '`' => {
                let code = collect_until(&mut chars, '`')?;
                inlines.push(Inline::Code(code));
            }
            '[' => {
                let text = collect_until(&mut chars, ']')?;
                if let Some('(') = chars.next() {
                    let href = collect_until(&mut chars, ')')?;
                    inlines.push(Inline::Link {
                        text: parse_inlines(&text)?,
                        href,
                    });
                } else {
                    Err(MdParseError::new(
                        "(<href>)",
                        chars.next().unwrap_or_default(),
                    ))?;
                }
            }
            _ => {
                let mut text = String::new();
                text.push(c);
                let mut escaped = false;
                while let Some(&nc) = chars.peek() {
                    if matches!(nc, '*' | '_' | '`' | '[') && !escaped {
                        break;
                    }
                    let next_c = chars.next().ok_or(MdParseError::new("a character", ""))?;
                    if next_c == '\\' && !escaped {
                        escaped = true;
                    } else {
                        escaped = false;
                        text.push(next_c);
                    }
                }
                inlines.push(Inline::Text(text));
            }
        }
    }

    Ok(inlines)
}

fn collect_until<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
    end: char,
) -> Result<String, MdParseError> {
    let mut s = String::new();
    for c in chars.by_ref() {
        if c == end {
            return Ok(s);
        }
        s.push(c);
    }
    Err(MdParseError::new(end, ""))
}

#[cfg(test)]
mod test {
    use crate::ast::Inline;

    use super::{collect_until, parse_inlines};

    #[test]
    fn collect_until_without_end() {
        let mut s = "abcdef".chars().peekable();
        let res = collect_until(&mut s, '.');
        assert!(res.is_err());
    }

    #[test]
    fn bold_text() {
        let md = "*abc*";
        let inl = parse_inlines(md).unwrap();

        assert_eq!(
            inl,
            vec![Inline::Bold(vec![Inline::Text("abc".to_string())])]
        );
    }

    #[test]
    fn italic_text() {
        let md = "_abc_";
        let inl = parse_inlines(md).unwrap();

        assert_eq!(
            inl,
            vec![Inline::Italic(vec![Inline::Text("abc".to_string())])]
        );
    }

    #[test]
    fn bold_italic_text() {
        let md = "*_abc_*";
        let inl = parse_inlines(md).unwrap();

        assert_eq!(
            inl,
            vec![Inline::Bold(vec![Inline::Italic(vec![Inline::Text(
                "abc".to_string()
            )])])]
        );
    }

    #[test]
    fn code() {
        let md = "`sudo rm -rf /`";
        let inl = parse_inlines(md).unwrap();

        assert_eq!(inl, vec![Inline::Code("sudo rm -rf /".to_string())]);
    }

    #[test]
    fn text_and_code() {
        let md = "run `sudo rm -rf /` on your computer";
        let inl = parse_inlines(md).unwrap();

        assert_eq!(
            inl,
            vec![
                Inline::Text("run ".to_string()),
                Inline::Code("sudo rm -rf /".to_string()),
                Inline::Text(" on your computer".to_string())
            ]
        );
    }

    #[test]
    fn single_hyperlink() {
        let md = "a link to [my site](https://example.com)";
        let inl = parse_inlines(md).unwrap();

        assert_eq!(
            inl,
            vec![
                Inline::Text("a link to ".to_string()),
                Inline::Link {
                    text: vec![Inline::Text("my site".to_string())],
                    href: "https://example.com".to_string()
                }
            ]
        );
    }

    #[test]
    fn hyperlink_without_link() {
        let md = "[abc]";
        let inl = parse_inlines(md);

        assert!(inl.is_err());
    }

    #[test]
    fn escape_brackets() {
        let md = r"some \[text\]";
        let inl = parse_inlines(md).unwrap();
        assert_eq!(inl, vec![Inline::Text("some [text]".to_string())]);
    }

    #[test]
    fn escape_escape() {
        let md = r"backslash \\";
        let inl = parse_inlines(md).unwrap();
        assert_eq!(inl, vec![Inline::Text(r"backslash \".to_string())]);
    }
}
