//! The module for number literals.

use crate::tokenizer::{
    token::{span::Span, TokenValue},
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
pub struct Number<'int, 'fract> {
    /// The base/radix of the number.
    pub base: Base,
    /// The integer part of the number.
    pub int_part: &'int [char],
    /// The fractional part of the number.
    ///
    /// `None` if the number is an integer.
    pub fract_part: Option<&'fract [char]>,
}

impl<'a> Tokenize<'a> for Number<'a, 'a> {
    // TODO: implement this, don't change how the number tokenization works, you can add exponents
    fn tokenize(chars: &'a [char], iter: &mut InputTextIter<'a>) -> TokenizeResult<'a> {
        if let Some(v) = iter.peek(0) {
            todo!("implement this, don't change how the number tokenization works, you can add exponents")
        } else {
            TokenizeResult::Eof
        }
    }
}
