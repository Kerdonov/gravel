/*

use crate::MdParseError;

pub type Pattern<T> = Vec<PatternToken<T>>;

pub enum PatternToken<T> {
    Once(T),
    Optional(T),
    AtLeastOnce(T),
    NTimes(T),
}

/// panics: on invalid pattern
pub fn char_pattern(s: &str) -> Pattern<char> {
    let mut s_chars = s.chars().peekable();
    let mut pat: Pattern<char> = Vec::new();
    while let Some(token) = s_chars.next() {
        pat.push(if let Some(&next) = s_chars.peek() {
            match next {
                '?' => {
                    s_chars.next().unwrap();
                    PatternToken::Optional(token)
                }
                '+' => {
                    s_chars.next().unwrap();
                    PatternToken::AtLeastOnce(token)
                }
                '*' => {
                    s_chars.next().unwrap();
                    PatternToken::NTimes(token)
                }
                _ => PatternToken::Once(token),
            }
        } else {
            PatternToken::Once(token)
        });
    }
    pat
}

pub trait ParsePattern: Iterator + Clone {
    fn parse<T>(&mut self, expect: Pattern<T>) -> Result<Vec<Self::Item>, MdParseError>
    where
        T: PartialEq<<Self as Iterator>::Item>,
    {
        let mut consumed = Vec::new();
        let mut cloned = self.clone();

        for pat_token in expect {
            match pat_token {
                PatternToken::Once(c) => {
                    if !cloned.next().map(|v| c == v).unwrap_or(false) {
                        return None;
                    }
                }
                PatternToken::Optional(c) => if cloned.peek().map(|v| c == *v).unwrap_or(false) {},
            }
        }

        *self = cloned;

        Some(consumed)
    }
}
*/

pub trait Parse: Iterator + Clone {
    fn follows(&mut self, token: char) -> bool;

    fn parse_token(&mut self, token: char) -> bool {
        if self.follows(token) {
            let _ = self.next();
            true
        } else {
            false
        }
    }

    fn parse_str(&mut self, tokens: &str) -> bool {
        let mut cloned = self.clone();

        for pat_token in tokens.chars() {
            if cloned.follows(pat_token) {
                cloned.next();
            } else {
                return false;
            }
        }

        *self = cloned;

        true
    }
}

impl Parse for std::iter::Peekable<std::str::Chars<'_>> {
    fn follows(&mut self, token: char) -> bool {
        self.peek().is_some_and(|c| c == &token)
    }
}

impl Parse for std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'_>>> {
    fn follows(&mut self, token: char) -> bool {
        self.peek().is_some_and(|&(_i, c)| c == token)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chars_parse_follows_double() {
        let mut c = "abc".chars().peekable();

        assert!(c.follows('a'));
        assert!(c.follows('a'));
    }

    #[test]
    fn chars_parse_tokens() {
        let mut c = "abcdef".chars().peekable();

        assert!(c.parse_token('a'));
        assert!(c.parse_token('b'));
    }

    #[test]
    fn chars_parse_str() {
        let mut c = "abcdef".chars().peekable();

        assert!(c.parse_str("abc"));
        assert!(c.parse_str("def"));
    }

    #[test]
    fn enumerate_parse_follows_double() {
        let mut c = "abc".chars().enumerate().peekable();

        assert!(c.follows('a'));
        assert!(c.follows('a'));
    }

    #[test]
    fn enumerate_parse_tokens() {
        let mut c = "abcdef".chars().enumerate().peekable();

        assert!(c.parse_token('a'));
        assert!(c.parse_token('b'));
    }

    #[test]
    fn enumerate_parse_str() {
        let mut c = "abcdef".chars().enumerate().peekable();

        assert!(c.parse_str("abc"));
        assert!(c.parse_str("def"));
    }

    #[test]
    fn enumerate_parse_token_failed_not_consume() {
        let mut c = "abc".chars().enumerate().peekable();

        assert!(!c.parse_token('b'));
        assert!(c.parse_token('a'));
    }

    #[test]
    fn enumerate_parse_str_failed_not_consume() {
        let mut c = "abcdef".chars().enumerate().peekable();

        assert!(!c.parse_str("def"));
        assert!(c.parse_str("abc"));
    }
}
