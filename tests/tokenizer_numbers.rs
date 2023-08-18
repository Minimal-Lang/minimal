use minimal_compiler::tokenizer::token::{
    literal::{Base, Number},
    TokenValue,
};
use util::TestTokenizer;

use crate::util::str_to_chars;

mod util;

#[test]
fn test_tokenizer_int() {
    let test = TestTokenizer::new("0_123456789_");

    assert_eq!(test[0].lexeme, &str_to_chars("0_123456789_"));
    assert_eq!(
        test[0].value,
        TokenValue::Number(Number {
            base: Base::Decimal,
            int_part: &str_to_chars("0_123456789_"),
            fract_part: None
        })
    );
}

#[test]
fn test_tokenizer_fract() {
    let test = TestTokenizer::new("0_123456789_.0_123456789_");

    assert_eq!(test[0].lexeme, &str_to_chars("0_123456789_.0_123456789_"));
    assert_eq!(
        test[0].value,
        TokenValue::Number(Number {
            base: Base::Decimal,
            int_part: &str_to_chars("0_123456789_"),
            fract_part: Some(&str_to_chars("0_123456789_"))
        })
    );
}
