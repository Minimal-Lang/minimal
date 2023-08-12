use std::mem::ManuallyDrop as M;

union ForceCaster<T, U> {
    t: M<T>,
    u: M<U>,
}

/// Force casting between values.
pub trait ForceCast<U>: Sized {
    /// A utility method for force casting between different types using a union.
    ///
    /// This method allows you to perform a force cast from type `Self` to `U`.
    /// The force cast reinterprets the bits of the source type as the destination type.
    ///
    /// # Safety
    ///
    /// This function is safe to use as long as `Self` and `U` are of the same size.
    /// If `Self` is smaller than `U`, behavior is *undefined*.
    /// If `Self` is larger than `U`, data will be truncated and corrupted.
    ///
    /// If you want a function that does the same but makes sure both types are of the
    /// same size, use [`core::mem::transmute`] or [`std::mem::transmute`].
    unsafe fn force_cast(self) -> U {
        let mut caster = ForceCaster { t: M::new(self) };

        // Safety: `caster.t` has to be dropped because `self` is consumed
        unsafe { M::drop(&mut caster.t) }

        unsafe { M::into_inner(caster.u) }
    }
}

impl<T, U> ForceCast<U> for T {}

/// A utility function for force casting between different types using a union.
///
/// This function allows you to perform a force cast between types `Src` and `Dst`.
/// The force cast reinterprets the bits of the source type as the destination type.
///
/// # Safety
///
/// This function is safe to use as long as `Src` and `Dst` are of the same size.
/// If `Src` is smaller than `Dst`, behavior is *undefined*.
/// If `Src` is larger than `Dst`, data will be truncated and corrupted.
///
/// If you want a function that does the same but makes sure both types are of the
/// same size, use [`core::mem::transmute`] or [`std::mem::transmute`].
pub unsafe fn force_cast<Src: ForceCast<Dst>, Dst>(src: Src) -> Dst {
    src.force_cast()
}
