//! The Minimal programming language lexical analyzer.
//!
//! Handles parsing of tokens, contains the [`InputTextIter`] type, and the [`Lexer`] struct.

use crate::{
    lexer::{
        parse::{Parse, ParseResult},
        token::{delim::Delim, ident::Ident, literal, operator::Operator, span::Span},
    },
    util::iter::Iter,
};

use self::token::Token;

pub mod parse;
pub mod token;

/// The type of the [`Lexer`](struct@Lexer)'s input iterator.
///
/// This iterator lets you see into the next character and index without advancing.
///
pub type InputTextIter<'text> = Iter<'text, char>;

/// The Minimal language lexer.
///
/// # Examples
/// ```rust
/// # use minimal_compiler::lexer::Lexer;
/// let input_code = &stringify!(Hello, "world"!).chars().collect::<Vec<char>>();
/// let lexer = Lexer::new(input_code);
/// ```
#[derive(Debug, Clone)]
pub struct Lexer<'input> {
    chars: &'input [char],
    iter: InputTextIter<'input>,
}

macro_rules! parse {
    (TODO $self:expr => $t:ty) => {{}};
    ($self:expr => $t:ty) => {{
        match <$t as Parse>::parse($self.chars, &mut $self.iter) {
            ParseResult::Token {
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
            ParseResult::Eof => return None,
            _ => (),
        }
    }};
}

impl<'input> Lexer<'input> {
    /// Creates a new lexer with specified input.
    ///
    /// This function only creates a [`Lexer`], it doesn't start the lexical analysis process.
    #[must_use = "calling `new()` creates a new `Lexer`, which must be used"]
    pub fn new(chars: &'input [char]) -> Self {
        Self {
            chars,
            iter: Iter::from_slice(chars),
        }
    }
    /// Gets the next token. Equivalent to `.next()` in iterating.
    pub fn next_token(&mut self) -> Option<Token<'input>> {
        let peek = self.iter.peek(0)?;
        if peek.1.is_whitespace() {
            let ret = Some(Token {
                value: token::TokenValue::Whitespace(*peek.1),
                lexeme: &self.chars[peek.0..=peek.0],
                span: Span {
                    from: peek.0,
                    to: peek.0,
                },
            });
            self.iter.next();
            return ret;
        }

        parse!(/* TODO */ self => literal::String);
        parse!(self => Ident);
        parse!(self => Delim);
        parse!(self => Operator);

        // Numbers come last to save a tiny little bit memory
        parse!(self => literal::Number);

        None
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    /// Gets the next token.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
