//! The trait and result type for tokenizing text.

use std::{fmt::Debug, ops::Range};

use super::{
    token::{Token, TokenValue},
    InputTextIter,
};

/// Trait for tokenizing text.
pub trait Tokenize<'text>
where
    Self: Debug + Clone,
{
    /// Tokenizes text.
    ///
    /// Parameter `text` is the text, parameter `iter` is the iterator, both from the same source.
    ///
    /// Returns [`TokenizeResult::NoMatch`] if the iterator didn't match,
    /// [`TokenizeResult::Eof`] if the iterator (text) ended too soon,
    /// [`TokenizeResult::Token`] if the iterator did match.
    ///
    /// The iterator is consumed if it matches, isn't if it doesn't.
    #[must_use]
    fn tokenize(chars: &'text [char], iter: &mut InputTextIter<'text>) -> TokenizeResult<'text>;
}

/// Result type for the [`tokenize`](fn@Tokenize::tokenize) function from the [`Tokenize`] trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizeResult<'text> {
    /// A token, returned if text matched.
    ///
    /// `lexeme` is the part of the text that contains the whole token,
    /// `value` is the value,
    /// `span` is the span.
    Token {
        /// The whole token as a character array.
        lexeme: &'text [char],
        /// The value of the token as an enum variant.
        value: TokenValue<'text>,
        /// The span of the token.
        span: Range<usize>,

        /// The errors in the token
        errors: Option<Vec<Token<'text>>>,
    },
    /// End of input, returned if the attempt to get the first character of the token was `None`.
    Eof,
    /// Text didn't match, iterator wasn't advanced.
    NoMatch,
}
