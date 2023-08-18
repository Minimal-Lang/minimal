//! The module for operator (e.g. `+`, `?`, `=`, `!`) tokens.

use crate::tokenizer::{
    token::{span::Span, TokenValue},
    tokenize::{Tokenize, TokenizeResult},
};

/// An operator (e.g. '+', '?', '!') token.
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
        Some(TokenizeResult::Token {
            lexeme: &$chars[$lexeme_and_span..=$lexeme_and_span],
            value: TokenValue::Operator(Operator::$name),
            span: Span {
                from: $lexeme_and_span,
                to: $lexeme_and_span,
            },
        })
    }};
}

impl<'text> Tokenize<'text> for Operator {
    fn tokenize(
        chars: &'text [char],
        iter: &mut crate::tokenizer::InputTextIter<'text>,
    ) -> crate::tokenizer::tokenize::TokenizeResult<'text> {
        if let Some(v) = iter.peek(0) {
            let v0 = v.0;
            let val = match *v.1 {
                '&' => pattern!(iter, chars, v0 => Ampersand),
                '@' => pattern!(iter, chars, v0 => At),

                '!' => pattern!(iter, chars, v0 => Bang),
                '?' => pattern!(iter, chars, v0 => QuestionMark),

                '=' => pattern!(iter, chars, v0 => Equal),

                '+' => pattern!(iter, chars, v0 => Plus),
                '-' => pattern!(iter, chars, v0 => Minus),
                '/' => pattern!(iter, chars, v0 => Slash),
                '*' => pattern!(iter, chars, v0 => Asterisk),
                '%' => pattern!(iter, chars, v0 => Percent),

                _ => None,
            };
            val.unwrap_or(TokenizeResult::NoMatch)
        } else {
            TokenizeResult::Eof
        }
    }
}
