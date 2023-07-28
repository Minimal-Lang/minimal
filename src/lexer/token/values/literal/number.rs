//! The module for number literals.

use crate::lexer::{
    parse::{Parse, ParseResult},
    InputTextIter,
};

/// A base/radix of a number.
#[doc(alias = "radix")]
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Binary (base 2)
    Binary = b'b',
    /// Decimal (base 8)
    Octal = b'o',

    /// Decimal (base 10)
    Decimal = b'\0',

    /// Hexadecimal (base 16)
    Hexadecimal = b'x',
}

/// A number token, represents any integer/float.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number<'int, 'fract> {
    /// The base/radix of the number.
    pub base: Base,
    /// The integer part of the number.
    pub int_part: &'int str,
    /// The fractional part of the number.
    pub fract_part: Option<&'fract str>,
}

impl<'int, 'fract> Number<'int, 'fract> {
    /// A new [`Number`].
    #[inline(always)]
    #[must_use = "called `new()`, which has to be used"]
    pub fn new(base: Base, int_part: &'int str, fract_part: Option<&'fract str>) -> Self {
        Self {
            base,
            int_part,
            fract_part,
        }
    }
}

impl<'a> Parse<'a> for Number<'a, 'a> {
    fn parse(chars: &'a [char], iter: &mut InputTextIter<'a>) -> ParseResult<'a> {
        todo!()
    }
}
