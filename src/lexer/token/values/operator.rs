//! The module for operator (e.g. `+`, `?`, `=`, `!`) tokens.

use crate::lexer::{
    parse::{Parse, ParseResult},
    token::{span::Span, TokenValue},
};

/// A delimiter (e.g. semicolons, commas, brackets) token.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    /// `&`
    Ampersand,
    /// `@`
    At,

    /// `!`
    Bang,
    /// `?`
    QuestionMark,

    /// `=`
    Equal,

    /// `+`
    Plus,

    /// `-`
    Minus,

    /// `/`
    Slash,

    /// `*`
    Asterisk,

    /// `%`
    Percent,
}

macro_rules! pattern {
    ($iter:expr, $chars:expr, $lexeme_and_span:expr => $name:ident) => {{
        $iter.next();
        Some(ParseResult::Token {
            lexeme: &$chars[$lexeme_and_span..=$lexeme_and_span],
            value: TokenValue::Operator(Operator::$name),
            span: Span {
                from: $lexeme_and_span,
                to: $lexeme_and_span,
            },
        })
    }};
}

impl<'text> Parse<'text> for Operator {
    fn parse(
        chars: &'text [char],
        iter: &mut crate::lexer::InputTextIter<'text>,
    ) -> crate::lexer::parse::ParseResult<'text> {
        if let Some(v) = iter.peek(1) {
            let v0 = v.0;
            let val = match *v.1 {
                '&' => pattern!(iter, chars, v0 => Ampersand),
                '@' => pattern!(iter, chars, v0 => At),

                '!' => pattern!(iter, chars, v0 => Bang),
                '?' => pattern!(iter, chars, v0 => QuestionMark),

                '+' => pattern!(iter, chars, v0 => Plus),
                '-' => pattern!(iter, chars, v0 => Minus),
                '/' => pattern!(iter, chars, v0 => Slash),
                '*' => pattern!(iter, chars, v0 => Asterisk),
                '%' => pattern!(iter, chars, v0 => Percent),

                _ => None,
            };
            val.unwrap_or(ParseResult::NoMatch)
        } else {
            ParseResult::Eof
        }
    }
}
