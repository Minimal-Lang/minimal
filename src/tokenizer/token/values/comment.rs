//! A comment token, line or block, doc or regular, not recusive.

use crate::tokenizer::{
    token::{span::Span, Error, TokenValue},
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
                    span: Span {
                        from: $start,
                        to: $chars.len() - 1,
                    },
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
                    span: Span {
                        from: $start,
                        to: $chars.len() - 1,
                    },
                };
            }
        }
    };
}

impl<'text> Tokenize<'text> for Comment<'text> {
    fn tokenize(chars: &'text [char], iter: &mut InputTextIter<'text>) -> TokenizeResult<'text> {
        if let Some(v) = iter.peek(0) {
            let '/' = v.1 else { return TokenizeResult::NoMatch };

            let (mut content_start, block) = match iter.peek(1) {
                Some((idx, '/')) => (idx + 1, false),
                Some((idx, '*')) => (idx + 1, true),
                _ => return TokenizeResult::NoMatch,
            };

            let start = v.0;

            let v = get_v!(iter.nth(2) => iter, block, chars, start, v, content_start);

            let doc = match block {
                true => *v.1 == '*',
                false => *v.1 == '/',
            };

            iter.next();

            if doc {
                content_start = v.0 + 1;
            }

            let end = loop {
                let Some(_) = iter.peek(0) else {
                    if block {
                        return TokenizeResult::Token {
                            lexeme: &chars[start..chars.len()],
                            value: TokenValue::Error(Error::UnterminatedBlockComment),
                            span: Span {
                                from: start,
                                to: chars.len(),
                            },
                        }
                    } else {
                        break chars.len();
                    }
                };
                iter.next();
                if let Some(v) = iter.peek(0) {
                    if block {
                        if let '*' = v.1 {
                            if let Some((idx, '/')) = iter.peek(1) {
                                break idx;
                            }
                        }
                    } else if let '\n' = v.1 {
                        break v.0;
                    }
                } else {
                    break chars.len();
                }
            };

            let content = &chars[content_start..end];

            TokenizeResult::Token {
                lexeme: &chars[start..end],
                value: TokenValue::Comment(Comment {
                    block,
                    doc,
                    content,
                }),
                span: Span {
                    from: start,
                    to: end,
                },
            }
        } else {
            TokenizeResult::Eof
        }
    }
}
