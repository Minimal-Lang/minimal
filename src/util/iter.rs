//! An iterator over a slice, used in the compiler.

use std::fmt::Debug;

use super::add_usize_isize;

/// An iterator over a slice.
///
/// Like [`Peekable`](struct@core::iter::Peekable) but it only works with a slice and
/// you can [`peek`](fn@Iter::peek) into the future or past as many items as you want.
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

    /// Peeks `n` times into the future (0 is the current element), or past if `n` is negative.
    pub fn peek(&self, n: isize) -> Option<(usize, &'a T)> {
        let idx = add_usize_isize(self.idx, n)?;
        if idx >= self.slice.len() {
            None
        } else {
            Some((idx, &self.slice[idx]))
        }
    }
}
