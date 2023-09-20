//! Character unescaping (e.g. `\\` changes to `\`)

use crate::util::parse_numbers::{parse_digit, parse_str_bin_byte, parse_str_hex_byte};

/// The result of unescaping.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnescapeResult {
    /// The result of unescaping.
    pub res: Result<char, UnescapeError>,
    /// The length of the escape sequence.
    pub len: usize,
}

/// An unescape error.
///
/// Unescape errors are invalid escapes.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnescapeError {
    /// A backslash `\` is the last character.
    EofAfterBackslash,
    /// An invalid escape.
    InvalidEscape,
    /// An invalid hexadecimal escape.
    InvalidHexEscape,
    /// An invalid binary escape.
    InvalidBinEscape,
    /// An invalid unicode escape
    InvalidUnicodeEscape,
    /// A unicode escape out of range.
    UnicodeEscapeOutOfRange,
}

/// Unescapes a single escape.
///
/// The backlash `\` is not a part of `chars`.
pub fn unescape(chars: &[char]) -> UnescapeResult {
    let len = chars.len();
    if len == 0 {
        return UnescapeResult {
            res: Err(UnescapeError::EofAfterBackslash),
            len: 0,
        };
    }
    // regular escapes
    match chars[0] {
        '0' => {
            return UnescapeResult {
                res: Ok('\0'),
                len: 1,
            }
        }
        'r' => {
            return UnescapeResult {
                res: Ok('\r'),
                len: 1,
            }
        }
        'n' => {
            return UnescapeResult {
                res: Ok('\n'),
                len: 1,
            }
        }
        't' => {
            return UnescapeResult {
                res: Ok('\t'),
                len: 1,
            }
        }
        '\'' => {
            return UnescapeResult {
                res: Ok('\''),
                len: 1,
            }
        }
        '\"' => {
            return UnescapeResult {
                res: Ok('\"'),
                len: 1,
            }
        }
        '\\' => {
            return UnescapeResult {
                res: Ok('\\'),
                len: 1,
            }
        }
        _ => (),
    }

    // hex escapes
    if chars[0] == 'x' {
        if len < 3 {
            return UnescapeResult {
                res: Err(UnescapeError::InvalidHexEscape),
                len,
            };
        }

        if chars[0].is_ascii_hexdigit() && chars[1].is_ascii_hexdigit() {
            return UnescapeResult {
                res: Ok(parse_str_hex_byte(&chars[1..=2]) as char),
                len: 2,
            };
        } else {
            return UnescapeResult {
                res: Err(UnescapeError::InvalidHexEscape),
                len: 3,
            };
        }
    }

    // binary escapes
    if chars[0] == 'b' {
        if len < 9 {
            return UnescapeResult {
                res: Err(UnescapeError::InvalidBinEscape),
                len,
            };
        }

        if !matches!(chars[1], '0' | '1')
            || !matches!(chars[2], '0' | '1')
            || !matches!(chars[3], '0' | '1')
            || !matches!(chars[4], '0' | '1')
            || !matches!(chars[5], '0' | '1')
            || !matches!(chars[6], '0' | '1')
            || !matches!(chars[7], '0' | '1')
            || !matches!(chars[8], '0' | '1')
        {
            return UnescapeResult {
                res: Err(UnescapeError::InvalidBinEscape),
                len,
            };
        }

        return UnescapeResult {
            res: Ok(parse_str_bin_byte(&chars[1..=8]) as char),
            len: 8,
        };
    }

    // unicode escapes
    if chars[0] == 'u' {
        if len < 4 {
            return UnescapeResult {
                res: Err(UnescapeError::InvalidUnicodeEscape),
                len,
            };
        }

        if len == 5
            && chars[1].is_ascii_hexdigit()
            && chars[2].is_ascii_hexdigit()
            && chars[3].is_ascii_hexdigit()
            && chars[4].is_ascii_hexdigit()
        {
            let mut ch = (parse_str_hex_byte(&chars[1..=2]) as u32) * 256;
            ch += parse_str_hex_byte(&chars[3..=4]) as u32;

            #[allow(clippy::transmute_int_to_char)]
            let ch: char = unsafe { core::mem::transmute(ch) };

            return UnescapeResult {
                res: Ok(ch),
                len: 5,
            };
        }

        if chars[1] != '{' {
            return UnescapeResult {
                res: Err(UnescapeError::InvalidUnicodeEscape),
                len: 2,
            };
        }

        let mut result = 0;

        let mut count = 0;

        for c in &chars[2..] {
            if count >= 8 && *c == '{' {
                break;
            }

            if !c.is_ascii_hexdigit() && count == 0 {
                return UnescapeResult {
                    res: Err(UnescapeError::InvalidUnicodeEscape),
                    len: 3,
                };
            }

            let digit_value = parse_digit::<16>(*c) as u32;

            result *= 16;
            result += digit_value;

            count += 1
        }

        if count == 0 {
            return UnescapeResult {
                res: Err(UnescapeError::InvalidUnicodeEscape),
                len: 3,
            };
        }

        match char::from_u32(result) {
            Some(v) => {
                return UnescapeResult {
                    res: Ok(v),
                    len: count + 1,
                }
            }
            None => {
                return UnescapeResult {
                    res: Err(UnescapeError::UnicodeEscapeOutOfRange),
                    len: count + 1,
                }
            }
        };
    }

    UnescapeResult {
        res: Err(UnescapeError::InvalidEscape),
        len: 1,
    }
}
