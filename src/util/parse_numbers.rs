//! Utilites for parsing numbers

/// Parses a single digit of a given radix/base (up to 16).
///
/// Any invalid character returns 0.
#[inline]
pub fn parse_digit<const RADIX: u8>(ch: char) -> u8 {
    match ch {
        '0' if RADIX >= 1 => 0x0,
        '1' if RADIX >= 2 => 0x1,
        '2' if RADIX >= 3 => 0x2,
        '3' if RADIX >= 4 => 0x3,
        '4' if RADIX >= 5 => 0x4,
        '5' if RADIX >= 6 => 0x5,
        '6' if RADIX >= 7 => 0x6,
        '7' if RADIX >= 8 => 0x7,
        '8' if RADIX >= 9 => 0x8,
        '9' if RADIX >= 10 => 0x9,
        'A' | 'a' if RADIX >= 11 => 0xA,
        'B' | 'b' if RADIX >= 12 => 0xB,
        'C' | 'c' if RADIX >= 13 => 0xC,
        'D' | 'd' if RADIX >= 14 => 0xD,
        'E' | 'e' if RADIX >= 15 => 0xE,
        'F' | 'f' if RADIX >= 16 => 0xF,
        _ => 0,
    }
}

/// Parses up to 2 hex digits into a `u8`.
///
/// `src` can have any number of characters, but max 2 are read.
/// If characters in `src` are invalid, they will count as 0.
pub fn parse_str_hex_byte(src: &[char]) -> u8 {
    let len = src.len();
    if len == 0 {
        0
    } else if len == 1 {
        parse_digit::<16>(src[0])
    } else {
        (parse_digit::<16>(src[1]) * 16) + parse_digit::<16>(src[0])
    }
}

/// Parses up to 8 binary digits into a `u8`.
///
/// `src` can have any number of characters, but max 8 are read.
/// If characters in `src` are invalid, they will count as 0.
#[rustfmt::skip]
pub fn parse_str_bin_byte(src: &[char]) -> u8 {
    let len = src.len();

    let mut ret = if len >= 1 { parse_digit::<2>(src[0]) } else { return 0; };
    if len >= 2 { ret *= 2; ret += parse_digit::<2>(src[1]); } else { return ret; }
    if len >= 3 { ret *= 2; ret += parse_digit::<2>(src[2]); } else { return ret; }
    if len >= 4 { ret *= 2; ret += parse_digit::<2>(src[3]); } else { return ret; }
    if len >= 5 { ret *= 2; ret += parse_digit::<2>(src[4]); } else { return ret; }
    if len >= 6 { ret *= 2; ret += parse_digit::<2>(src[5]); } else { return ret; }
    if len >= 7 { ret *= 2; ret += parse_digit::<2>(src[6]); } else { return ret; }
    if len >= 8 { ret *= 2; ret += parse_digit::<2>(src[7]); } else { return ret; }

    ret
}
