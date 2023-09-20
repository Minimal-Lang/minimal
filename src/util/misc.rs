/// Adds an `isize` to a `usize` value (checked).
pub fn add_usize_isize(usize_val: usize, isize_val: isize) -> Option<usize> {
    use core::mem::transmute;

    if isize_val >= 0 {
        unsafe { usize_val.checked_add(transmute(isize_val)) }
    } else {
        unsafe { usize_val.checked_sub(transmute(isize_val.abs())) }
    }
}
