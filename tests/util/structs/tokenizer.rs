use minimal_compiler::tokenizer::{token::Token, Tokenizer};

use core::ops::Index;

/// A struct for testing the lexer.
#[derive(Debug)]
pub struct TestTokenizer<'a> {
    /// The output of the lexer.
    pub output: Vec<Token<'a>>,
}

impl<'a> TestTokenizer<'a> {
    /// Creates a new [`TestTokenizer`] from `&[char]`.
    pub fn from_chars(text: &'a [char]) -> Self {
        Self {
            output: Vec::from_iter(Tokenizer::new(text)),
        }
    }
}

impl<'a> Index<usize> for TestTokenizer<'a> {
    type Output = Token<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.output[index]
    }
}
