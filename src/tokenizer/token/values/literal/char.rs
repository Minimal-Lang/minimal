//! The module for character literals.

use crate::{
    tokenizer::{
        token::{Error, Token, TokenValue},
        tokenize::{Tokenize, TokenizeResult},
        InputTextIter,
    },
    util::unescape::unescape,
};

/// A character literal.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char {
    /// The unescaped value under the char
    pub char: char,
}

impl<'c> Tokenize<'c> for Char {
    fn tokenize(chars: &'c [char], iter: &mut InputTextIter<'c>) -> TokenizeResult<'c> {
        let v = if let Some(v) = iter.next() {
            v
        } else {
            return TokenizeResult::Eof;
        };

        if *v.1 != '\'' {
            return TokenizeResult::NoMatch;
        }

        let start = v.0;

        let v = if let Some(v) = iter.next() {
            v
        } else {
            return TokenizeResult::Token {
                lexeme: &chars[start..=start],
                value: TokenValue::Error(Error::UnterminatedCharacterLiteral),
                span: start..start + 1,
                errors: None,
            };
        };

        let mut unescape_errors = Vec::new();

        let c = if *v.1 == '\\' {
            let unescaped = unescape(&chars[v.0 + 1..]);

            if unescaped.len != 0 {
                iter.nth(unescaped.len - 1);
            }

            match unescaped.res {
                Ok(v) => v,
                Err(e) => {
                    let end = v.0 + unescaped.len;
                    unescape_errors.push(Token {
                        lexeme: &chars[start..end],
                        value: TokenValue::UnescapeError(e),
                        span: start..v.0 + unescaped.len,
                    });
                    '\0'
                }
            }
        } else if *v.1 == '\'' {
            return TokenizeResult::Token {
                lexeme: &chars[start..v.0],
                value: TokenValue::Error(Error::EmptyCharacterLiteral),
                span: start..v.0,
                errors: None,
            };
        } else {
            *v.1
        };

        if let Some(v) = iter.next() {
            if *v.1 == '\'' {
                return TokenizeResult::Token {
                    lexeme: &chars[start..v.0],
                    value: TokenValue::Character(Char { char: c }),
                    span: start..v.0,
                    errors: Some(unescape_errors),
                };
            }
            for (i, c) in iter {
                if *c == '\'' {
                    return TokenizeResult::Token {
                        lexeme: &chars[start..i],
                        value: TokenValue::Error(Error::CharacterLiteralTooLong),
                        span: start..i,
                        errors: Some(unescape_errors),
                    };
                }
            }
        }

        TokenizeResult::Token {
            lexeme: &chars[start..],
            value: TokenValue::Error(Error::UnterminatedCharacterLiteral),
            span: start..chars.len(),
            errors: Some(unescape_errors),
        }
    }
}
