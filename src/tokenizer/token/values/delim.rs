//! The module for delimiter (e.g. semicolons, commas, brackets) tokens.

use crate::tokenizer::{
    tokenize::{Tokenize, TokenizeResult},
    InputTextIter,
};

use super::TokenValue;

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
        Some(TokenizeResult::Token {
            lexeme: &$chars[$lexeme_and_span..=$lexeme_and_span],
            value: TokenValue::Delim(Delim::$name),
            span: $lexeme_and_span..$lexeme_and_span + 1,
            errors: None,
        })
    }};
}

impl<'text> Tokenize<'text> for Delim {
    fn tokenize(chars: &'text [char], iter: &mut InputTextIter<'text>) -> TokenizeResult<'text> {
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
                TokenizeResult::NoMatch
            }
        } else {
            TokenizeResult::Eof
        }
    }
}
