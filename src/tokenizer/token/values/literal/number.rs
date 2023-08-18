//! The module for number literals.

use crate::tokenizer::{
    token::{span::Span, Error, TokenValue},
    tokenize::{Tokenize, TokenizeResult},
    InputTextIter,
};

/// A base/radix of a number.
#[doc(alias = "radix")]
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Binary (base 2)
    Binary,
    /// Octal (base 8)
    Octal,

    /// Decimal (base 10)
    Decimal,

    /// Hexadecimal (base 16)
    Hexadecimal,
}

/// A number token, represents any integer/float.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number<'int, 'fract, 'exponent> {
    /// The base/radix of the number.
    pub base: Base,
    /// The integer part of the number.
    pub int_part: &'int [char],
    /// The fractional part of the number.
    ///
    /// `None` if the number is an integer.
    pub fract_part: Option<&'fract [char]>,
    /// The exponent of the number.
    ///
    /// `None` if the number is an integer.
    pub exponent: Option<&'exponent [char]>,
}

/// Used in the implementation of Tokenize
fn number_matches_base(number: char, base: &Base) -> bool {
    match base {
        Base::Binary => matches!(number, '0'..='1' | '_'),
        Base::Octal => matches!(number, '0'..='7' | '_'),
        Base::Decimal => matches!(number, '0'..='9' | '_'),
        Base::Hexadecimal => matches!(number, '0'..='9' | 'a'..='f' | 'A'..='F' | '_'),
    }
}

impl<'a> Tokenize<'a> for Number<'a, 'a, 'a> {
    fn tokenize(chars: &'a [char], iter: &mut InputTextIter<'a>) -> TokenizeResult<'a> {
        if let Some(v) = iter.peek(0) {
            // TODO: get the base (handle invalid inputs)
            // TODO: get the integer part (handle invalid inputs)
            // TODO: get the fractional part (handle invalid inputs)
            // TODO: get the exponent (handle invalid inputs)

            todo!()
        } else {
            TokenizeResult::Eof
        }
    }
}
