//! The module for string literals.

use crate::tokenizer::{
    token::{self, span::Span, TokenValue},
    tokenize::{Tokenize, TokenizeResult},
};

/// A string token value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct String<'s> {
    /// The unescaped value under the string.
    pub s: &'s str,
}

impl<'s> Tokenize<'s> for String<'s> {
    fn tokenize(
        text: &'s [char],
        iter: &mut crate::tokenizer::InputTextIter<'s>,
    ) -> TokenizeResult<'s> {
        if let Some(v) = iter.peek(0) {
            if *v.1 == '\"' {
                let start_idx = v.0;
                let mut end_idx = v.0;
                iter.next();

                let mut string = std::string::String::new();

                let mut ended = false;

                while let Some(v) = iter.next() {
                    end_idx = v.0;
                    match *v.1 {
                        '"' => {
                            ended = true;
                            break;
                        }
                        '\\' => match iter.next() {
                            _ => (),
                        }, // TODO: finish escape sequences
                        _ => string.push(*v.1),
                    }
                }
                if ended {
                    TokenizeResult::Token {
                        lexeme: &text[start_idx..=end_idx],
                        value: TokenValue::String(String {
                            s: Box::leak(Box::new(string)),
                        }),
                        span: Span {
                            from: start_idx,
                            to: end_idx,
                        },
                    }
                } else {
                    TokenizeResult::Token {
                        lexeme: &text[start_idx..=end_idx],
                        value: TokenValue::Error(token::Error::UnterminatedStringLiteral),
                        span: Span {
                            from: start_idx,
                            to: end_idx,
                        },
                    }
                }
            } else {
                TokenizeResult::NoMatch
            }
        } else {
            TokenizeResult::Eof
        }
    }
}
