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
    #[allow(clippy::manual_is_ascii_check)]
    match base {
        Base::Binary => matches!(number, '0'..='1' | '_'),
        Base::Octal => matches!(number, '0'..='7' | '_'),
        Base::Decimal => matches!(number, '0'..='9' | '_'),
        Base::Hexadecimal => matches!(number, '0'..='9' | 'a'..='f' | 'A'..='F' | '_'),
    }
}

impl<'a> Tokenize<'a> for Number<'a, 'a> {
    // TODO: Rust hates me, can anybody else make this work???????????????????
    fn tokenize(chars: &'a [char], iter: &mut InputTextIter<'a>) -> TokenizeResult<'a> {
        if let Some(v) = iter.peek(0) {
            let start = v.0;

            let (base, has_base_prefix) = if let '0' = v.1 {
                if let v @ Some('b' | 'o' | 'x') = iter.peek(1).map(|v| v.1) {
                    iter.nth(2);
                    (
                        match unsafe { v.unwrap_unchecked() } {
                            'b' => Base::Binary,
                            'o' => Base::Octal,
                            'x' => Base::Hexadecimal,
                            _ => unreachable!(),
                        },
                        true,
                    )
                } else {
                    (Base::Decimal, false)
                }
            } else {
                (Base::Decimal, false)
            };

            let int_part_start = match iter.peek(0) {
                Some(v) => v.0,
                None if has_base_prefix => {
                    return TokenizeResult::Token {
                        lexeme: &chars[start..v.0],
                        value: TokenValue::Error(Error::NoNumberAfterBase),
                        span: Span {
                            from: start,
                            to: v.0,
                        },
                    }
                }
                _ => unreachable!(),
            };

            let mut int_part_end = 0;
            #[allow(clippy::while_let_loop)]
            loop {
                let v = match iter.peek(0) {
                    Some(v) => {
                        iter.next();

                        int_part_end = v.0;

                        v
                    }
                    None => break,
                };
                if !number_matches_base(*v.1, &base) {
                    break;
                }
            }

            if int_part_start == int_part_end {
                // It's guaranteed the base prefix isn't here
                return TokenizeResult::NoMatch;
            }

            let int_part = &chars[int_part_start..=int_part_end];

            let fract_part_start = if let Some((_, '.')) = iter.peek(0) {
                match iter.peek(1) {
                    Some((idx, c)) if number_matches_base(*c, &base) => {
                        iter.nth(2);
                        Some(idx)
                    }
                    _ => None,
                }
            } else {
                None
            };

            #[allow(clippy::while_let_loop)]
            let fract_part_end = fract_part_start.map(|_| {
                let mut fract_part_end = 0;
                loop {
                    let v = match iter.peek(0) {
                        Some(v) => {
                            iter.next();

                            fract_part_end = v.0;

                            v
                        }
                        None => break,
                    };
                    if !number_matches_base(*v.1, &base) {
                        break;
                    }
                }
                fract_part_end
            });

            let fract_part = fract_part_start
                .map(|fract_part_start| &chars[fract_part_start..=fract_part_end.unwrap()]);

            let end = fract_part_end.unwrap_or(int_part_end);

            if let Some(fract_part) = fract_part {
                TokenizeResult::Token {
                    lexeme: &chars[start..end],
                    value: TokenValue::Number(Number {
                        base,
                        int_part,
                        fract_part: Some(fract_part),
                    }),
                    span: Span {
                        from: start,
                        to: end,
                    },
                }
            } else {
                TokenizeResult::Token {
                    lexeme: &chars[start..end],
                    value: TokenValue::Number(Number {
                        base,
                        int_part,
                        fract_part: None,
                    }),
                    span: Span {
                        from: start,
                        to: end,
                    },
                }
            }
        } else {
            TokenizeResult::Eof
        }
    }
}
