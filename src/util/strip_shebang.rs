/// Strips the shebang (`#!...`) off of the first line of text.
///
/// Only works with line endings that end with LF.
pub fn strip_shebang(text: &[char]) -> &[char] {
    if text.len() <= 2 {
        return text;
    }

    if text[0] == '#' && text[1] == '!' {
        let mut shebang_end_idx = text.len();
        for (idx, c) in text[2..].iter().enumerate() {
            if *c == '\n' {
                shebang_end_idx = idx;
                break;
            }
        }
        if text.len() <= shebang_end_idx {
            return &[];
        }
        &text[shebang_end_idx + 3..]
    } else {
        text
    }
}
