//! The module for string literals.

use crate::lexer::{
    parse::{Parse, ParseResult},
    token::{self, span::Span, TokenValue},
};

/// A string token value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct String<'s> {
    /// The unescaped value under the string.
    pub s: &'s str,
}

impl<'s> Parse<'s> for String<'s> {
    fn parse(text: &'s [char], iter: &mut crate::lexer::InputTextIter<'s>) -> ParseResult<'s> {
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
                    ParseResult::Token {
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
                    ParseResult::Token {
                        lexeme: &text[start_idx..=end_idx],
                        value: TokenValue::Error(token::Error::UnterminatedStringLiteral),
                        span: Span {
                            from: start_idx,
                            to: end_idx,
                        },
                    }
                }
            } else {
                ParseResult::NoMatch
            }
        } else {
            ParseResult::Eof
        }
    }
}
