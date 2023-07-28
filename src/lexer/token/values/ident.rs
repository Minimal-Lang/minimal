//! The module for identfier tokens, including keywords.

use crate::lexer::parse::{Parse, ParseResult};

use super::{span::Span, TokenValue};

/// An identifier, or keyword.
///
/// An `Ident` starts with an underscore or a letter,
/// rest is either an underscore, a letter, or a number.
///
/// Whether an `Ident` is a keyword or just an identifier is determined by the parser.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ident<'ident> {
    value: &'ident [char],
}

impl<'ident> Parse<'ident> for Ident<'ident> {
    fn parse(
        chars: &'ident [char],
        iter: &mut crate::lexer::InputTextIter<'ident>,
    ) -> ParseResult<'ident> {
        if let Some(v) = iter.peek(0) {
            if v.1.is_alphabetic() || *v.1 == '_' {
                let start_idx = v.0;
                let mut end_idx = chars.len();

                iter.next();

                while let Some(v) = iter.peek(0) {
                    if !(v.1.is_alphanumeric() || *v.1 == '_') {
                        end_idx = v.0;

                        break;
                    }
                    iter.next();
                }

                let lexeme = &chars[start_idx..end_idx];

                ParseResult::Token {
                    lexeme,
                    value: TokenValue::Ident(Self { value: lexeme }),
                    span: Span {
                        from: start_idx,
                        to: end_idx,
                    },
                }
            } else {
                ParseResult::NoMatch
            }
        } else {
            ParseResult::Eof
        }
    }
}
