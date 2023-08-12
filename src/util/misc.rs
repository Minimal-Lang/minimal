/// Adds an `isize` to a `usize` value (checked).
pub fn add_usize_isize(usize_val: usize, isize_val: isize) -> Option<usize> {
    use crate::util::force_cast::force_cast;

    if isize_val >= 0 {
        unsafe { usize_val.checked_add(force_cast::<isize, usize>(isize_val)) }
    } else {
        // `r#usize` minus negative (makes it positive) `r#isize`
        unsafe { usize_val.checked_sub(force_cast::<isize, usize>(-isize_val)) }
    }
}
