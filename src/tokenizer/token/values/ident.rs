//! The module for identfier tokens, including keywords.

use crate::tokenizer::tokenize::{Tokenize, TokenizeResult};

use super::{span::Span, TokenValue};

/// An identifier, or keyword.
///
/// An `Ident` starts with an underscore or a letter,
/// rest is either an underscore, a letter, or a number.
///
/// Whether an `Ident` is a keyword or just an identifier is determined by the parser.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ident<'ident> {
    /// The value of the ident, same as the lexeme.
    pub value: &'ident [char],
}

impl<'ident> Tokenize<'ident> for Ident<'ident> {
    fn tokenize(
        chars: &'ident [char],
        iter: &mut crate::tokenizer::InputTextIter<'ident>,
    ) -> TokenizeResult<'ident> {
        if let Some(v) = iter.peek(0) {
            if v.1.is_alphabetic() || *v.1 == '_' {
                let start_idx = v.0;
                let mut end_idx = chars.len();

                iter.next();

                while let Some(v) = iter.peek(0) {
                    if !(v.1.is_alphanumeric() || *v.1 == '_') {
                        end_idx = v.0 - 1;

                        break;
                    }
                    iter.next();
                }

                let lexeme = &chars[start_idx..end_idx];

                TokenizeResult::Token {
                    lexeme,
                    value: TokenValue::Ident(Self { value: lexeme }),
                    span: Span {
                        from: start_idx,
                        to: end_idx,
                    },
                }
            } else {
                TokenizeResult::NoMatch
            }
        } else {
            TokenizeResult::Eof
        }
    }
}
