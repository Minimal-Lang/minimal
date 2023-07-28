//! An iterator over text, for the lexer.

use std::fmt::Debug;

/// An iterator over a slice.
///
/// Like [`Peekable`](struct@core::iter::Peekable), but you can [`peek`](fn&Iter::peek) into the future as many tokens as you want.
#[derive(Clone)]
pub struct Iter<'a, T>
where
    T: 'a,
{
    slice: &'a [T],

    idx: usize,
}

impl<'a, T> Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Iter").finish_non_exhaustive()
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: 'a,
{
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.idx > self.slice.len() {
            None
        } else {
            Some((self.idx, &self.slice[self.idx]))
        };
        self.idx += 1;
        ret
    }
}

impl<'a, T> Iter<'a, T> {
    /// Turns a slice into an [`Iter`].
    pub fn from_slice(slice: &'a [T]) -> Self {
        Self { slice, idx: 0 }
    }
    /// Peeks `n` times into the future, or past if `n` is negative.
    pub fn peek(&self, n: isize) -> Option<(usize, &'a T)> {
        if (self.idx as isize + n) > self.slice.len() as isize || self.idx as isize + n < 0 {
            None
        } else {
            Some((self.idx, &self.slice[(self.idx as isize + n) as usize]))
        }
    }
}
