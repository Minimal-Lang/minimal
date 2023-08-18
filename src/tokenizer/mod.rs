//! The Minimal programming language lexical analyzer.
//!
//! Handles parsing of tokens, contains the [`InputTextIter`] type, and the [`Tokenizer`] struct.

use crate::tokenizer::{
    token::{
        comment::Comment, delim::Delim, ident::Ident, literal, operator::Operator, span::Span,
    },
    tokenize::{Tokenize, TokenizeResult},
};

use self::token::Token;

pub mod token;
pub mod tokenize;

/// The type of the [`Tokenizer`](struct@Tokenizer)'s input iterator.
///
/// This iterator lets you see into the future, past, and present without advancing.
pub type InputTextIter<'text> = crate::util::iter::Iter<'text, char>;

/// The Minimal language tokenizer.
///
/// # Examples
///
/// ```rust
/// # use minimal_compiler::tokenizer::Tokenizer;
/// let input_code = &stringify!(Hello, "world"!).chars().collect::<Vec<char>>();
/// let tokenizer = Tokenizer::new(input_code);
/// ```
#[derive(Debug, Clone)]
pub struct Tokenizer<'input> {
    chars: &'input [char],
    iter: InputTextIter<'input>,
}

macro_rules! tokenize {
    (TODO $self:expr => $t:ty) => {{}};
    ($self:expr => $t:ty) => {{
        match <$t as Tokenize>::tokenize($self.chars, &mut $self.iter) {
            TokenizeResult::Token {
                value,
                lexeme,
                span,
            } => {
                return Some(Token {
                    lexeme,
                    value,
                    span,
                })
            }
            TokenizeResult::Eof => return None,
            _ => (),
        }
    }};
}

impl<'input> Tokenizer<'input> {
    /// Creates a new tokenizer with specified input.
    ///
    /// This function only creates a [`Tokenizer`], it doesn't start the lexical analysis process.
    #[must_use = "calling `new()` creates a new `Tokenizer`, which must be used"]
    pub fn new(chars: &'input [char]) -> Self {
        Self {
            chars,
            iter: InputTextIter::from_slice(chars),
        }
    }
    /// Gets the next token. Equivalent to `.next()` in iterating (that's why it's private).
    fn next_token(&mut self) -> Option<Token<'input>> {
        // Removes whitespaces.
        let peek = self.iter.peek(0)?;
        if peek.1.is_whitespace() {
            self.iter.next();
            return Some(Token {
                value: token::TokenValue::Whitespace(*peek.1),
                lexeme: &self.chars[peek.0..=peek.0],
                span: Span {
                    from: peek.0,
                    to: peek.0,
                },
            });
        }

        // Comments have to go first to prevent it being treated
        // as operators.
        // TODO: create the Comment struct, implement the Tokenize trait for it, document it, remove the TODO from the below line, test it
        tokenize!(self => Comment);

        // Order doesn't really matter but it's best if kept
        // in this order; from least complex to most complex.

        tokenize!(self => Delim);
        tokenize!(self => Operator);

        tokenize!(self => Ident);
        tokenize!(self => literal::String);

        // Numbers are considered more complex than strings.
        tokenize!(self => literal::Number);

        None // FIXME: should be invalid token error instead
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Token<'input>;

    /// Gets the next token.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
