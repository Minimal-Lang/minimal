use minimal_compiler::tokenizer::token::{comment::Comment, TokenValue};
use util::TestTokenizer;

use crate::util::str_to_chars;

mod util;

#[test]
fn test_tokenizer_line_comment() {
    let test = TestTokenizer::new("// This is a comment");

    assert_eq!(test[0].lexeme, &str_to_chars("// This is a comment"));
    assert_eq!(
        test[0].value,
        TokenValue::Comment(Comment {
            doc: false,
            block: false,
            content: &str_to_chars(" This is a comment")
        })
    );
}

#[test]
fn test_tokenizer_line_doc_comment() {
    let test = TestTokenizer::new("/// This is a doc comment");

    assert_eq!(test[0].lexeme, &str_to_chars("/// This is a doc comment"));
    assert_eq!(
        test[0].value,
        TokenValue::Comment(Comment {
            doc: true,
            block: false,
            content: &str_to_chars(" This is a doc comment")
        })
    );
}

#[test]
fn test_tokenizer_block_comment() {
    let test = TestTokenizer::new(
        "/* This is a block comment,
        and it continues... */",
    );

    assert_eq!(
        test[0].lexeme,
        &str_to_chars(
            "/* This is a block comment,
        and it continues... */"
        )
    );
    assert_eq!(
        test[0].value,
        TokenValue::Comment(Comment {
            doc: true,
            block: false,
            content: &str_to_chars(
                " This is a block comment,
        and it continues... "
            )
        })
    );
}
