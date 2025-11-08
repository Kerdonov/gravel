use crate::ast::Inline;

pub fn parse_inlines(input: &str) -> Vec<Inline> {
    let mut inlines = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '*' => {
                let inner = collect_until(&mut chars, '*');
                inlines.push(Inline::Bold(parse_inlines(&inner)));
            }
            '_' => {
                let inner = collect_until(&mut chars, '_');
                inlines.push(Inline::Italic(parse_inlines(&inner)));
            }
            '`' => {
                let code = collect_until(&mut chars, '`');
                inlines.push(Inline::Code(code));
            }
            '[' => {
                let text = collect_until(&mut chars, ']');
                if chars.next() == Some('(') {
                    let href = collect_until(&mut chars, ')');
                    inlines.push(Inline::Link {
                        text: parse_inlines(&text),
                        href,
                    });
                }
            }
            _ => {
                let mut text = String::new();
                text.push(c);
                while let Some(&nc) = chars.peek() {
                    if matches!(nc, '*' | '_' | '`' | '[') {
                        break;
                    }
                    text.push(chars.next().unwrap());
                }
                inlines.push(Inline::Text(text));
            }
        }
    }

    inlines
}

fn collect_until<I: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<I>,
    end: char,
) -> String {
    let mut s = String::new();
    while let Some(&c) = chars.peek() {
        if c == end {
            chars.next();
            break;
        }
        s.push(chars.next().unwrap());
    }
    s
}
