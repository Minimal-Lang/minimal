//! A token, output of the tokenizer, input of the parser.

use std::ops::Range;

use crate::util::unescape::UnescapeError;

#[path = "values/delim.rs"]
pub mod delim;
#[path = "values/ident.rs"]
pub mod ident;
#[path = "values/operator.rs"]
pub mod operator;

#[path = "values/literal/mod.rs"]
pub mod literal;

#[path = "values/comment.rs"]
pub mod comment;

/// A token, output of the tokenizer, input of the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    /// The whole token as a string
    pub lexeme: &'a [char],
    /// The value of the token of an enum variant.
    pub value: TokenValue<'a>,
    /// The span of the token.
    pub span: Range<usize>,
}

/// The value of a token.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenValue<'a> {
    /// A whitespace character.
    Whitespace,

    /// A comment.
    Comment(comment::Comment<'a>),

    /// An indentifier or keyword.
    Ident(ident::Ident<'a>),

    /// A number literal (integer or floating point).
    ///
    /// The tokenizer **doesn't** make sure number works with its radix
    Number(literal::Number<'a, 'a>),
    /// A string literal.
    String(literal::String),
    /// A character literal.
    Character(literal::Char),

    /// A delimiter, like brackets and colons.
    Delim(delim::Delim),
    /// An operator, like `+`, `=`, `?`, `!`.
    Operator(operator::Operator),

    /// A tokenization error.
    Error(Error),

    /// An error while unescaping.
    UnescapeError(UnescapeError),
}

/// A lexical analysis error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// An invalid character.
    InvalidCharacter,

    /// An unterminated string literal.
    UnterminatedStringLiteral,

    /// An unterminated character literal.
    UnterminatedCharacterLiteral,

    /// A too long character literal (more than one character).
    CharacterLiteralTooLong,

    /// An empty character literal
    EmptyCharacterLiteral,

    /// No number after base prefix in number literal.
    NoNumberAfterBase,

    /// No number after exponent sign in number literal.
    NoNumberAfterExponentSign,

    /// Unterminated block comment.
    UnterminatedBlockComment,
}
