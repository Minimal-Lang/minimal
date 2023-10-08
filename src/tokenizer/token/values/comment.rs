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

impl<'text> Tokenize<'text> for Comment<'text> {
    fn tokenize(chars: &'text [char], iter: &mut InputTextIter<'text>) -> TokenizeResult<'text> {
        let (start, block) = match (iter.peek(0), iter.peek(1)) {
            (None, _) => return TokenizeResult::Eof,
            (Some((start, '/')), Some((_, block @ ('/' | '*')))) => {
                iter.nth(1);

                let block = match block {
                    '/' => false,
                    '*' => true,
                    _ => unreachable!(),
                };
                (start, block)
            }
            (Some(_), _) => return TokenizeResult::NoMatch,
        };

        let mut content_start = start + 1;
        let mut content_end = content_start;
        let mut end = content_start;

        while let Some(v) = iter.next() {
            // if content_start is not initialized
            if content_start == start + 1 {
                content_start = v.0
            }
            todo!(
                "handle edge cases such as unterminated block comments and eof inside line comment"
            );
        }

        todo!();
    }
}
