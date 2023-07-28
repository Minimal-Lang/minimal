#[test]
pub fn test_lexer_unicode() {
    let chars = &r#"
pub fn ahoj() {
    println!("Hello, world!");
}
"#
    .chars()
    .collect::<Vec<char>>();

    let lexer = minimal_compiler::lexer::Lexer::new(chars);

    for token in lexer {
        println!("{:?}", token);
    }

    panic!("necum")
}
