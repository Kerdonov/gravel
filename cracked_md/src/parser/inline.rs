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
                while let Some(nc) = chars.next() {
                    if matches!(nc, '*' | '_' | '`' | '[') {
                        break;
                    }
                    text.push(nc);
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
    while let Some(c) = chars.next() {
        if c == end {
            return Ok(s);
        }
        s.push(c);
    }
    Err(MdParseError::new(end, ""))
}
