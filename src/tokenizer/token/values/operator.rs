//! The module for operator (e.g. `+`, `?`, `=`, `!`) tokens.

use crate::tokenizer::{
    token::TokenValue,
    tokenize::{Tokenize, TokenizeResult},
};

/// An operator (e.g. '+', '?', '!') token.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    /// `&`
    Ampersand,
    /// `&&`
    BitAnd,
    /// `&=`
    AmpersandEqual,

    /// `|`
    Pipe,
    /// `||`
    BitOr,
    /// `|=`
    PipeEqual,

    /// `@`
    At,

    /// `!`
    Bang,
    /// `!=`
    BangEqual,

    /// `?`
    QuestionMark,

    /// `=`
    Equal,

    /// `+`
    Plus,
    /// `+=`
    PlusEqual,

    /// `-`
    Minus,
    /// `+=`
    MinusEqual,

    /// `/`
    Slash,
    /// `+=`
    SlashEqual,

    /// `*`
    Asterisk,
    /// `+=`
    AsteriskEqual,

    /// `%`
    Percent,
    /// `+=`
    PercentEqual,

    /// `<`
    LessThan,
    /// `<=`
    LessThanEqual,
    /// `<<`
    ShiftLeft,

    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanEqual,
    /// `>>`
    ShiftRight,
}

macro_rules! pattern {
    ($iter:expr, $chars:expr, $lexeme_and_span:expr => $name:ident) => {{
        $iter.next();
        Some(TokenizeResult::Token {
            lexeme: &$chars[$lexeme_and_span..=$lexeme_and_span],
            value: TokenValue::Operator(Operator::$name),
            span: $lexeme_and_span..$lexeme_and_span + 1,
            errors: None,
        })
    }};
    ($iter:expr, $chars:expr, $lexeme_and_span_start:expr, $lexeme_and_span_end:expr; $next:literal => $name1:ident, $name2:ident) => {{
        $iter.next();
        if $iter.peek(0).is_some_and(|v| *v.1 == $next) {
            $iter.next();
            Some(TokenizeResult::Token {
                lexeme: &$chars[$lexeme_and_span_start..=$lexeme_and_span_end],
                value: TokenValue::Operator(Operator::$name2),
                span: $lexeme_and_span_start..$lexeme_and_span_end + 1,
                errors: None,
            })
        } else {
            Some(TokenizeResult::Token {
                lexeme: &$chars[$lexeme_and_span_start..=$lexeme_and_span_end],
                value: TokenValue::Operator(Operator::$name1),
                span: $lexeme_and_span_start..$lexeme_and_span_end + 1,
                errors: None,
            })
        }
    }};
    ($iter:expr, $chars:expr, $lexeme_and_span_start:expr, $lexeme_and_span_end:expr; $next1:literal | $next2:literal => $name1:ident, $name2:ident | $name3:ident) => {{
        $iter.next();
        if let Some((_, v @ $next1 | v @ $next2)) = $iter.peek(0) {
            if *v == $next1 {
                $iter.next();
                Some(TokenizeResult::Token {
                    lexeme: &$chars[$lexeme_and_span_start..=$lexeme_and_span_end],
                    value: TokenValue::Operator(Operator::$name2),
                    span: $lexeme_and_span_start..$lexeme_and_span_end + 1,
                    errors: None,
                })
            } else {
                Some(TokenizeResult::Token {
                    lexeme: &$chars[$lexeme_and_span_start..=$lexeme_and_span_end],
                    value: TokenValue::Operator(Operator::$name3),
                    span: $lexeme_and_span_start..$lexeme_and_span_end + 1,
                    errors: None,
                })
            }
        } else {
            Some(TokenizeResult::Token {
                lexeme: &$chars[$lexeme_and_span_start..=$lexeme_and_span_end],
                value: TokenValue::Operator(Operator::$name1),
                span: $lexeme_and_span_start..$lexeme_and_span_end + 1,
                errors: None,
            })
        }
    }};
}

impl<'text> Tokenize<'text> for Operator {
    fn tokenize(
        chars: &'text [char],
        iter: &mut crate::tokenizer::InputTextIter<'text>,
    ) -> crate::tokenizer::tokenize::TokenizeResult<'text> {
        let v = if let Some(v) = iter.peek(0) {
            v
        } else {
            return TokenizeResult::Eof;
        };
        let v0 = v.0;
        let val = match *v.1 {
            '&' => {
                pattern!(iter, chars, v0, v0 + 1; '&' | '=' => Ampersand, BitAnd | AmpersandEqual)
            }
            '|' => {
                pattern!(iter, chars, v0, v0 + 1; '|' | '=' => Pipe, BitOr | PipeEqual)
            }

            '@' => pattern!(iter, chars, v0 => At),

            '!' => pattern!(iter, chars, v0, v0 + 1; '=' => Bang, BangEqual),

            '?' => pattern!(iter, chars, v0 => QuestionMark),

            '=' => pattern!(iter, chars, v0 => Equal),

            '+' => pattern!(iter, chars, v0, v0 + 1; '=' => Plus, PlusEqual),
            '-' => pattern!(iter, chars, v0, v0 + 1; '=' => Minus, MinusEqual),
            '/' => pattern!(iter, chars, v0, v0 + 1; '=' => Slash, SlashEqual),
            '*' => pattern!(iter, chars, v0, v0 + 1; '=' => Asterisk, AsteriskEqual),
            '%' => pattern!(iter, chars, v0, v0 + 1; '=' => Percent, PercentEqual),

            '<' => {
                pattern!(iter, chars, v0, v0 + 1; '=' | '<' => LessThan, LessThanEqual | ShiftLeft)
            }
            '>' => {
                pattern!(iter, chars, v0, v0 + 1; '=' | '>' => GreaterThan, GreaterThanEqual | ShiftRight)
            }
            _ => None,
        };
        val.unwrap_or(TokenizeResult::NoMatch)
    }
}
