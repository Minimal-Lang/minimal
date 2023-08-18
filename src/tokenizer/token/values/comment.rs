use crate::tokenizer::{
    tokenize::{Tokenize, TokenizeResult},
    InputTextIter,
};

/// A comment, line or block, doc or regular, not recusive.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Comment {
    /// Whether or not the comment is a documentation comment.
    doc: bool,
    /// Whether or not the comment is a block comment (`/* */`).
    block: bool,
}

impl<'text> Tokenize<'text> for Comment {
    fn tokenize(chars: &'text [char], iter: &mut InputTextIter<'text>) -> TokenizeResult<'text> {
        if let Some(v) = iter.peek(0) {
            // TODO: finish comments
            todo!("finish comments");
        } else {
            TokenizeResult::Eof
        }
    }
}
