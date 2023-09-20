//! The module for number literals.

use crate::tokenizer::{
    token::{Error, TokenValue},
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

/// Used in the implementation of Tokenize
fn number_matches_base(number: char, base: &Base) -> bool {
    match base {
        Base::Binary => matches!(number, '0'..='1' | '_'),
        Base::Octal => matches!(number, '0'..='7' | '_'),
        Base::Decimal => matches!(number, '0'..='9' | '_'),
        Base::Hexadecimal => matches!(number, '0'..='9' | 'a'..='f' | 'A'..='F' | '_'),
    }
}

impl<'a> Tokenize<'a> for Number<'a, 'a> {
    fn tokenize(chars: &'a [char], iter: &mut InputTextIter<'a>) -> TokenizeResult<'a> {
        let mut v = if let Some(v) = iter.peek(0) {
            v
        } else {
            return TokenizeResult::Eof;
        };

        let start = v.0;

        let base = if *v.1 == '0' {
            if let Some((_, base @ ('b' | 'o' | 'x'))) = iter.peek(1) {
                v = match iter.nth(1) {
                    Some(v) => {
                        iter.next();
                        v
                    }
                    None => {
                        return TokenizeResult::Token {
                            lexeme: &chars[v.0..chars.len()],
                            value: TokenValue::Error(Error::NoNumberAfterBase),
                            span: v.0..chars.len(),
                            errors: None,
                        }
                    }
                };

                match base {
                    'b' => Base::Binary,
                    'o' => Base::Octal,
                    'x' => Base::Hexadecimal,
                    _ => unreachable!(),
                }
            } else {
                Base::Decimal
            }
        } else {
            Base::Decimal
        };

        let start_int = v.0;

        let mut end_int = start_int;

        while let Some(v) = iter.peek(0) {
            end_int = v.0;

            if !number_matches_base(*v.1, &base) {
                if end_int == start_int {
                    return TokenizeResult::NoMatch;
                }
                break;
            }
            iter.next();
        }

        let int_range = start_int..end_int;
        let int = &chars[int_range.clone()];

        // if the number only has an integer part
        // (it doesn't have a dot).
        let Some((_, '.')) = iter.peek(0) else {
            return TokenizeResult::Token { lexeme: &chars[start..end_int], value: TokenValue::Number(Number { base, int_part: int, fract_part: None, }), span: int_range, errors: None };
        };

        let v = if let Some(v) = iter.peek(1) {
            if number_matches_base(*v.1, &base) {
                iter.nth(1);
                v
            } else {
                return TokenizeResult::Token {
                    lexeme: &chars[start..end_int],
                    value: TokenValue::Number(Number {
                        base,
                        int_part: int,
                        fract_part: None,
                    }),
                    span: int_range,
                    errors: None,
                };
            }
        } else {
            return TokenizeResult::Token {
                lexeme: &chars[start..end_int],
                value: TokenValue::Number(Number {
                    base,
                    int_part: int,
                    fract_part: None,
                }),
                span: int_range,
                errors: None,
            };
        };

        let start_fract = v.0;
        let mut end_fract = v.0;

        while let Some(v) = iter.peek(0) {
            if !number_matches_base(*v.1, &base) {
                end_fract = v.0;
                break;
            }
            iter.next();
        }

        let fract_range = start_fract..end_fract;
        let fract = &chars[fract_range];

        TokenizeResult::Token {
            lexeme: &chars[start..end_fract],
            value: TokenValue::Number(Number {
                base,
                int_part: int,
                fract_part: Some(fract),
            }),
            span: start..end_fract,
            errors: None,
        }
    }
}
