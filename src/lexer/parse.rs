//! The trait and result type for parsing tokens.

use std::fmt::Debug;

use super::{
    token::{span::Span, TokenValue},
    InputTextIter,
};

/// Trait for converting text into tokens.
pub trait Parse<'text>
where
    Self: Debug + Clone,
{
    /// A function for converting text into a token.
    ///
    /// Returns [`ParseResult::NoMatch`] if the iterator didn't match,
    /// [`ParseResult::Eof`] if the attempt to get the first character of the token was `None`,
    /// [`ParseResult::Token`] if the iterator did match.
    ///
    /// Parameter `text` is the text, parameter `iter` is the iterator, both from the lexer.
    ///
    /// The iterator is consumed if it matches, isn't if it doesn't.
    #[must_use]
    fn parse(chars: &'text [char], iter: &mut InputTextIter<'text>) -> ParseResult<'text>;
}

/// Result type for the [`parse`](fn@Parse::parse) function from the [`Parse`] trait.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseResult<'text> {
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
        /// The [`Span`](struct@super::token::span::Span) of the token.
        span: Span,
    },
    /// End of input, returned if the attempt to get the first character of the token was `None`.
    Eof,
    /// Text didn't match, iterator wasn't advanced.
    NoMatch,
}
