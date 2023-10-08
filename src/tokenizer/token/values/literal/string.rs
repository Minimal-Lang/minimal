//! The module for string literals.

use std::string::String as StdString;

use crate::{
    tokenizer::{
        token::{self, Token, TokenValue},
        tokenize::{Tokenize, TokenizeResult},
    },
    util::unescape::unescape,
};

/// A string token value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct String {
    /// The unescaped value under the string.
    pub s: StdString,
}

impl<'s> Tokenize<'s> for String {
    fn tokenize(
        chars: &'s [char],
        iter: &mut crate::tokenizer::InputTextIter<'s>,
    ) -> TokenizeResult<'s> {
        let start_idx = match iter.peek(0) {
            Some((idx, '"')) => idx,
            Some(_) => return TokenizeResult::NoMatch,
            None => return TokenizeResult::Eof,
        };

        let mut end_idx = chars.len();
        iter.next();

        let mut string = StdString::new();
        let mut errors: Vec<Token<'s>> = Vec::new();

        while let Some(v) = iter.next() {
            end_idx = v.0;
            match *v.1 {
                '"' => {
                    return TokenizeResult::Token {
                        value: TokenValue::String(String { s: string }),
                        span: start_idx..end_idx,
                        errors: Some(errors),
                    };
                }
                '\\' => {
                    let unescaped = unescape(&chars[v.0 + 1..]);

                    match unescaped.len {
                        0 => (),
                        v => {
                            iter.nth(v - 1);
                        }
                    }

                    match unescaped.res {
                        Ok(v) => string.push(v),
                        Err(e) => errors.push(Token {
                            value: TokenValue::UnescapeError(e),
                            span: v.0 + 1..unescaped.len,
                        }),
                    }
                }
                _ => string.push(*v.1),
            }
        }

        TokenizeResult::Token {
            value: TokenValue::Error(token::Error::UnterminatedStringLiteral),
            span: start_idx..end_idx,
            errors: None,
        }
    }
}
