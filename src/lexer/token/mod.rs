//! A token, output of the lexer, input of the parser.

#[path = "values/delim.rs"]
pub mod delim;
#[path = "values/ident.rs"]
pub mod ident;
#[path = "values/operator.rs"]
pub mod operator;

#[path = "values/literal/mod.rs"]
pub mod literal;

pub mod span;

/// A token, output of the lexer, input of the parser.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'a> {
    /// The whole token as a string
    pub lexeme: &'a [char],
    /// The value of the token of an enum variant.
    pub value: TokenValue<'a>,
    /// The [`Span`](struct@span::Span) of the token.
    pub span: span::Span,
}

/// The value of a token.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenValue<'a> {
    /// A whitespace character.
    Whitespace(char),

    /// An indentifier or keyword.
    Ident(ident::Ident<'a>),
    /// A number literal, integer or floating point.
    Number(literal::Number<'a, 'a>),
    /// A string literal.
    String(literal::String<'a>),
    /// A delimiter, like brackets and colons.
    Delim(delim::Delim),
    /// An operator, like `+`, `=`, `?`, `!`.
    Operator(operator::Operator),

    /// A lexical analysis error.
    Error(Error),
}

/// A lexical analysis error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// An unterminated string literal.
    UnterminatedStringLiteral,

    /// An invalid number suffix.
    ///
    /// A number cannot end with arbitrary text.
    InvalidNumberSuffix,
}
