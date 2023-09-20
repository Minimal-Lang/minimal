//! A comment token, line or block, doc or regular, not recusive.

use crate::tokenizer::{
    token::{Error, TokenValue},
    tokenize::{Tokenize, TokenizeResult},
    InputTextIter,
};

/// A comment, line or block, doc or regular, not recusive.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Comment<'s> {
    /// Whether or not the comment is a documentation comment.
    pub doc: bool,
    /// Whether or not the comment is a block comment (`/* */`).
    pub block: bool,
    /// The content of the comment.
    pub content: &'s [char],
}

macro_rules! get_v {
    ($exp:expr => $iter:expr, $block:expr, $chars:expr, $start:expr, $v:expr, $content_start:expr) => {
        match $exp {
            Some(v) => {
                $iter.next();
                v
            }
            _ if $block => {
                return TokenizeResult::Token {
                    lexeme: &$chars[$start..$chars.len()],
                    value: TokenValue::Error(Error::UnterminatedBlockComment),
                    span: $start..$chars.len(),
                    errors: None,
                };
            }
            _ => {
                return TokenizeResult::Token {
                    lexeme: &$chars[$start..$v.0],
                    value: TokenValue::Comment(Comment {
                        block: $block,
                        doc: false,
                        content: &$chars[$content_start..$chars.len()],
                    }),
                    span: $start..$chars.len(),
                    errors: None,
                };
            }
        }
    };
}

impl<'text> Tokenize<'text> for Comment<'text> {
    fn tokenize(chars: &'text [char], iter: &mut InputTextIter<'text>) -> TokenizeResult<'text> {
        let (v, start) = if let Some(v @ (idx, '/')) = iter.peek(0) {
            (v, idx)
        } else {
            return TokenizeResult::NoMatch;
        };
        let (mut content_start, is_block) = match iter.peek(1) {
            Some((idx, '/')) => (idx + 1, false),
            Some((idx, '*')) => (idx + 1, true),
            _ => return TokenizeResult::NoMatch,
        };

        let v = get_v!(iter.nth(2) => iter, is_block, chars, start, v, content_start);

        let is_doc = match is_block {
            true => *v.1 == '*',
            false => *v.1 == '/',
        };

        iter.next();

        if is_doc {
            content_start = v.0 + 1;
        }

        let (end, content_end) = loop {
            if let Some(v) = iter.peek(0) {
                if is_block {
                    if let '*' = v.1 {
                        if let Some((idx, '/')) = iter.peek(1) {
                            iter.nth(1);
                            break (idx + 1, v.0);
                        }
                    }
                } else if let '\n' = v.1 {
                    break (v.0, v.0);
                }
            } else if is_block {
                return TokenizeResult::Token {
                    lexeme: &chars[start..chars.len()],
                    value: TokenValue::Error(Error::UnterminatedBlockComment),
                    span: start..chars.len(),
                    errors: None,
                };
            } else {
                break (chars.len(), chars.len());
            }
            iter.next();
        };

        let content = &chars[content_start..content_end];

        TokenizeResult::Token {
            lexeme: &chars[start..end],
            value: TokenValue::Comment(Comment {
                block: is_block,
                doc: is_doc,
                content,
            }),
            span: start..end,
            errors: None,
        }
    }
}
