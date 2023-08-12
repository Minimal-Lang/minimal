//! The module for delimiter (e.g. semicolons, commas, brackets) tokens.

use crate::lexer::{
    parse::{Parse, ParseResult},
    InputTextIter,
};

use super::{span::Span, TokenValue};

/// A delimiter (e.g. semicolons, commas, brackets) token.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Delim {
    /// `#`
    Hash,
    /// `;`
    SemiColon,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `.`
    Dot,

    /// `[`
    LBracket,
    /// `]`
    RBracket,

    /// `{`
    LBrace,
    /// `}`
    RBrace,

    /// `(`
    LParen,
    /// `)`
    RParen,
}

macro_rules! pattern {
    ($iter:expr, $chars:expr, $lexeme_and_span:expr => $name:ident) => {{
        $iter.next();
        Some(ParseResult::Token {
            lexeme: &$chars[$lexeme_and_span..=$lexeme_and_span],
            value: TokenValue::Delim(Delim::$name),
            span: Span {
                from: $lexeme_and_span,
                to: $lexeme_and_span,
            },
        })
    }};
}

impl<'text> Parse<'text> for Delim {
    fn parse(chars: &'text [char], iter: &mut InputTextIter<'text>) -> ParseResult<'text> {
        if let Some(v) = iter.peek(0) {
            let v0 = v.0;
            let val = match *v.1 {
                '#' => pattern!(iter, chars, v0 => Hash),

                ';' => pattern!(iter, chars, v0 => SemiColon),
                ':' => pattern!(iter, chars, v0 => Colon),
                ',' => pattern!(iter, chars, v0 => Comma),
                '.' => pattern!(iter, chars, v0 => Dot),

                '[' => pattern!(iter, chars, v0 => LBracket),
                ']' => pattern!(iter, chars, v0 => RBracket),

                '{' => pattern!(iter, chars, v0 => LBrace),
                '}' => pattern!(iter, chars, v0 => RBrace),

                '(' => pattern!(iter, chars, v0 => LParen),
                ')' => pattern!(iter, chars, v0 => RParen),

                _ => None,
            };
            if let Some(val) = val {
                val
            } else {
                ParseResult::NoMatch
            }
        } else {
            ParseResult::Eof
        }
    }
}
