use crate::ast::Block;

use super::inline::parse_inlines;

pub fn parse_blocks(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        if line.starts_with("#") {
            let level = line.chars().take_while(|&c| c == '#').count() as u8;
            let text = line[level as usize..].trim();
            blocks.push(Block::Heading {
                level,
                content: parse_inlines(text),
            });
        } else if let Some(quote_body) = line.strip_prefix(">") {
            let quote_blocks = parse_blocks(quote_body);
            blocks.push(Block::Quote(quote_blocks));
        } else if line.starts_with("```") {
            let lang_line = line.strip_prefix("```").unwrap().to_string();
            let lang = if lang_line.is_empty() {
                None
            } else {
                Some(lang_line)
            };
            let mut code = String::new();
            while lines.peek().is_some() && !lines.peek().unwrap().starts_with("```") {
                code.push_str(&format!("{}\n", lines.next().unwrap()));
            }
            lines.next();
            blocks.push(Block::Code {
                language: lang,
                content: code,
            });
        } else if line.trim().is_empty() {
            continue;
        } else {
            blocks.push(Block::Paragraph(parse_inlines(line)));
        }
    }

    blocks
}
