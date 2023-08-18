use minimal_compiler::tokenizer::{token::Token, Tokenizer};

use core::{
    mem::{align_of, size_of},
    ops::Index,
};
use std::alloc::{alloc, dealloc, Layout};

/// A struct for testing the lexer.
#[derive(Debug)]
pub struct TestTokenizer<'a> {
    ptr: *mut u8,
    layout: Layout,
    /// The output of the lexer.
    pub output: Vec<Token<'a>>,
}

impl<'a> TestTokenizer<'a> {
    /// Creates a new `TestLexer`.
    pub fn new(text: &'a str) -> Self {
        let char_count = text.chars().count();

        // SAFETY: The alignment is always right because the `align_of::<T>()` function guarantees it.
        let layout = unsafe {
            Layout::from_size_align_unchecked(char_count * size_of::<char>(), align_of::<char>())
        };

        // SAFETY: The layout is always correct.
        let ptr: *mut char = unsafe { alloc(layout) } as _;

        for (i, (_, c)) in text.char_indices().enumerate() {
            unsafe { *ptr.add(i) = c }
        }

        // SAFETY: the pointer is always correct because alloc should never fail.
        let slice: &[char] = unsafe { core::slice::from_raw_parts(ptr as _, char_count) };

        Self {
            ptr: ptr as _,
            layout,
            output: Tokenizer::new(slice).collect(),
        }
    }
}

impl<'a> Drop for TestTokenizer<'a> {
    fn drop(&mut self) {
        // SAFETY: The value is being dropped, it won't be used again.
        unsafe { dealloc(self.ptr, self.layout) }
    }
}

impl<'a> Index<usize> for TestTokenizer<'a> {
    type Output = Token<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.output[index]
    }
}
