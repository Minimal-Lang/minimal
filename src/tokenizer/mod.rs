//! The Minimal programming language lexical analyzer.
//!
//! Handles parsing of tokens, contains the [`InputTextIter`] type, and the [`Tokenizer`] struct.

use std::collections::VecDeque;

use crate::tokenizer::{
    token::{comment::Comment, delim::Delim, ident::Ident, literal, operator::Operator, Token},
    tokenize::{Tokenize, TokenizeResult},
};

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
/// let input_code = stringify!(Hello, "world"!).chars().collect::<Vec<char>>();
/// let tokenizer = Tokenizer::new(&input_code);
/// ```
#[derive(Debug, Clone)]
pub struct Tokenizer<'input> {
    chars: &'input [char],
    iter: InputTextIter<'input>,
    error_stack: Option<VecDeque<Token<'input>>>,
}

macro_rules! tokenize {
    (TODO $self:expr => $t:ty) => {{}};
    ($self:expr => $t:ty) => {{
        match <$t as Tokenize>::tokenize($self.chars, &mut $self.iter) {
            TokenizeResult::Token {
                value,
                span,
                errors,
            } => {
                if let Some(errors) = errors {
                    if errors.len() > 0 {
                        $self.error_stack = Some(VecDeque::from(errors));
                    }
                }
                return Some(Token { value, span });
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
    #[must_use]
    pub fn new(chars: &'input [char]) -> Self {
        Self {
            chars,
            iter: InputTextIter::from_slice(chars),
            error_stack: None,
        }
    }
    /// Gets the next token. Equivalent to `.next()` in iterating (that's why it's private).
    fn next_token(&mut self) -> Option<Token<'input>> {
        if let Some(error_stack) = &mut self.error_stack {
            match error_stack.pop_front() {
                None => self.error_stack = None,
                v @ Some(_) => return v,
            }
        }

        // Removes whitespaces.
        let peek = self.iter.peek(0)?;
        if peek.1.is_whitespace() {
            self.iter.next();
            return Some(Token {
                value: token::TokenValue::Whitespace,
                span: peek.0..peek.0 + 1,
            });
        }

        // Comments have to go first to prevent being treated
        // as operators.
        tokenize!(self => Comment);

        // Order doesn't really matter but it's best if kept
        // in this order; from least complex to most complex.

        tokenize!(self => Delim);

        tokenize!(self => Ident);

        tokenize!(self => Operator);

        tokenize!(self => literal::String);

        // Numbers are more complex than strings.
        tokenize!(self => literal::Number);

        if let Some((idx, _)) = self.iter.peek(0) {
            Some(Token {
                value: token::TokenValue::Error(token::Error::InvalidCharacter),
                span: idx..idx + 1,
            })
        } else {
            None
        }
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Token<'input>;

    /// Gets the next token.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
