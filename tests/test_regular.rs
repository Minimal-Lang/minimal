use minimal_compiler::util::strip_shebang;
use util::TestTokenizer;

mod util;

#[test]
fn test_regular() {
    let file = util::read_file_contents("./tests/test_regular.mn").expect(
        "file `./tests/test_regular.mn` not found (are you running this test from the crate root?)",
    );

    let code = util::exclude_comment_lines(&file);

    let code = strip_shebang(&code);

    let test = TestTokenizer::from_chars(code);

    dbg!(&code[2..23]);

    assert_token!(code, test[0], lexeme = "\n", span = 0..1, kind = whitespace);
    assert_token!(code, test[1], lexeme = "\n", span = 1..2, kind = whitespace);
    assert_token!(
        code,
        test[2],
        lexeme = "// This is a comment\n",
        span = 2..23,
        kind = comment,
        block = false,
        doc = false,
        content = " This is a comment"
    );
    assert_token!(
        code,
        test[3],
        lexeme = "\n",
        span = 23..24,
        kind = whitespace
    );
    assert_token!(
        code,
        test[4],
        lexeme = "/// This is a doc comment",
        span = 23..49,
        kind = comment,
        block = false,
        doc = true,
        content = " This is a doc comment"
    );
    assert_token!(
        code,
        test[5],
        lexeme = "\n",
        span = 49..50,
        kind = whitespace
    );
    assert_token!(
        code,
        test[6],
        lexeme = "/*/* This is a\n block comment*/*/",
        span = 50..83,
        kind = comment,
        block = true,
        doc = false,
        content = " This is a doc comment"
    );
    assert_token!(
        code,
        test[7],
        lexeme = "/*/* This is a\n block comment*/*/",
        span = 83..84,
        kind = comment,
        block = true,
        doc = true,
        content = " This is a doc comment"
    );
}
