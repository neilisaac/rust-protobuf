use std::mem;
use std::mem::MaybeUninit;
use std::slice;

/// `Vec::spare_capacity_mut` is not stable until Rust 1.60.
pub(crate) fn vec_spare_capacity_mut<A>(vec: &mut Vec<A>) -> &mut [MaybeUninit<A>] {
    // SAFETY: copy-paste from rust stdlib.
    unsafe {
        slice::from_raw_parts_mut(
            vec.as_mut_ptr().add(vec.len()) as *mut MaybeUninit<A>,
            vec.capacity() - vec.len(),
        )
    }
}

/// `MaybeUninit::write_slice` is not stable.
pub(crate) fn maybe_uninit_write_slice<'a, T>(
    this: &'a mut [MaybeUninit<T>],
    src: &[T],
) -> &'a mut [T]
where
    T: Copy,
{
    // SAFETY: copy-paste from rust stdlib.

    let uninit_src: &[MaybeUninit<T>] = unsafe { mem::transmute(src) };

    this.copy_from_slice(uninit_src);

    unsafe { &mut *(this as *mut [MaybeUninit<T>] as *mut [T]) }
}
