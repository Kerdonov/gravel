use super::inline::parse_inlines;
use crate::{MdParseError, ast::Block};

use crate::parse_trait::Parse;

pub fn parse_blocks(input: &str) -> Result<Vec<Block>, MdParseError> {
    let mut blocks = Vec::new();
    let mut lines = input.lines().enumerate().peekable();

    while let Some((i, line)) = lines.next() {
        let mut line_chars = line.chars().peekable();

        // header
        let mut heading_level = 0;
        while line_chars.parse_token('#') {
            if heading_level < 6 {
                heading_level += 1;
            }
        }
        if heading_level > 0 {
            if !line_chars.parse_token(' ') {
                Err(MdParseError::from_line(
                    i + 1,
                    "<space> after #",
                    "no <space>",
                ))?;
            }
            let line_content: String = line_chars.collect();
            blocks.push(Block::Heading {
                level: heading_level,
                content: parse_inlines(&line_content)?,
            });
            continue;
        }

        // quote TODO
        /*
        if line_chars.parse_str("> ") {
            let content: String = line_chars.collect();
            let quote_blocks = parse_blocks(&content).map_err(|e| e.set_line(i + 1))?;
            blocks.push(Block::Quote(quote_blocks));
            continue;
        }
        */

        // code
        if line_chars.parse_str("```") {
            let lang_line: String = line_chars.collect();
            let lang = if lang_line.is_empty() {
                None
            } else {
                Some(lang_line)
            };
            let mut code = String::new();

            for (j, line) in lines.by_ref() {
                let mut code_line_chars = line.chars().peekable();
                // code block end
                if code_line_chars.parse_str("```") {
                    let remaining: String = code_line_chars.collect();
                    if remaining.is_empty() {
                        blocks.push(Block::Code {
                            language: lang,
                            content: code,
                        });
                        break;
                    } else {
                        Err(MdParseError::from_line(
                            j + 1,
                            "```",
                            format!("```{}", remaining),
                        ))?;
                    }
                } else {
                    code.push_str(line);
                    code.push('\n');
                }
            }
            Err(MdParseError::from_line(i + 1, "a terminating '```'", ""))?;
        }

        // lists TODO
    }

    Ok(blocks)
}

/*
pub fn parse_blocks(input: &str) -> Result<Vec<Block>, MdParseError> {
    let mut blocks = Vec::new();

    let mut lines = input.lines().enumerate().peekable();

    while let Some((i, line)) = lines.next() {
        if line.starts_with("#") {
            let level = line.chars().take_while(|&c| c == '#').count() as u8;
            let text = line[level as usize..].trim();
            blocks.push(Block::Heading {
                level,
                content: parse_inlines(text).map_err(|e| e.set_line(i + 1))?,
            });
        } else if let Some(quote_body) = line.strip_prefix(">") {
            let quote_blocks = parse_blocks(quote_body).map_err(|e| e.set_line(i + 1))?;
            blocks.push(Block::Quote(quote_blocks));
        } else if line.starts_with("```") {
            let lang_line = line.strip_prefix("```").unwrap().to_string();
            let lang = if lang_line.is_empty() {
                None
            } else {
                Some(lang_line)
            };
            let mut code = String::new();
            while lines.peek().is_some()
                && !lines
                    .peek()
                    .ok_or(MdParseError::from_line(i + 1, "a line", ""))?
                    .1
                    .starts_with("```")
            {
                if let Some((_i, l)) = lines.next() {
                    code.push_str(&format!("{}\n", l));
                }
            }
            lines.next();
            blocks.push(Block::Code {
                language: lang,
                content: code,
            });
        } else if line.trim().is_empty() {
            continue;
        } else {
            blocks.push(Block::Paragraph(
                parse_inlines(line).map_err(|e| e.set_line(i + 1))?,
            ));
        }
    }

    Ok(blocks)
}
*/
