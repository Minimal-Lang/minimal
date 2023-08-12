//! The module for number literals.

use crate::lexer::{
    parse::{Parse, ParseResult},
    token::{span::Span, TokenValue},
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

impl<'a> Parse<'a> for Number<'a, 'a> {
    fn parse(chars: &'a [char], iter: &mut InputTextIter<'a>) -> ParseResult<'a> {
        if let Some(v) = iter.peek(0) {
            let (base, has_base_prefix) = if let Some(v) = iter.peek(0) {
                if *v.1 == '0' {
                    if let Some(c @ 'b' | c @ 'o' | c @ 'x') = iter.peek(1).map(|v| v.1) {
                        iter.nth(2);
                        (
                            match c {
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
                }
            } else {
                return ParseResult::NoMatch;
            };

            if let '0'..='9' = *v.1 {
                let int_part_start = v.0;
                let mut int_part_end = chars.len() - 1;
                let mut fract_part_start = 0;
                let mut fract_part_end = 0;

                iter.next();

                while let Some(v) = iter.peek(0) {
                    if let '0'..='9' | '_' = *v.1 {
                        iter.next();
                        int_part_end = v.0;
                    } else {
                        break;
                    }
                }

                // Check for a decimal point and parse fraction part if present
                if let Some(v) = iter.peek(0) {
                    if *v.1 == '.' {
                        fract_part_start = v.0 + 1;

                        iter.next();

                        // number with dot but no fract part (dot isn't a part)
                        let Some((_, '0'..='9' | '_')) = iter.peek(1) else {
                            return ParseResult::Token {
                                lexeme: &chars[int_part_start..=int_part_end],
                                value: TokenValue::Number(Number {
                                    base,
                                    int_part: &chars[int_part_start..=int_part_end],
                                    fract_part: None,
                                }),
                                span: Span {
                                    from: int_part_start,
                                    to: int_part_end,
                                },
                            }
                        };

                        while let Some(v) = iter.peek(0) {
                            if let '0'..='9' | '_' = *v.1 {
                                iter.next();
                                fract_part_end = v.0;
                            } else {
                                break;
                            }
                        }
                    }
                }

                // if the number has a fractional part
                if fract_part_start != 0 {
                    ParseResult::Token {
                        lexeme: &chars[int_part_start..=fract_part_end],
                        value: TokenValue::Number(Number {
                            base,
                            int_part: &chars[int_part_start..=int_part_end],
                            fract_part: Some(&chars[fract_part_start..=fract_part_end]),
                        }),
                        span: Span {
                            from: int_part_start,
                            to: fract_part_end,
                        },
                    }
                } else {
                    ParseResult::Token {
                        lexeme: &chars[int_part_start..=int_part_end],
                        value: TokenValue::Number(Number {
                            base,
                            int_part: &chars[int_part_start..=int_part_end],
                            fract_part: None,
                        }),
                        span: Span {
                            from: int_part_start,
                            to: int_part_end,
                        },
                    }
                }
            } else {
                ParseResult::NoMatch
            }
        } else {
            ParseResult::Eof
        }
    }
}
