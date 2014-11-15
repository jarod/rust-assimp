use std::raw;
use std::mem;

#[inline(always)]
pub unsafe fn ptr_ptr_to_slice<'a, T>(ptr: *mut*mut T, len: uint) -> &'a [&'a T] {
    let raw_slice : raw::Slice<&T> = raw::Slice {
        data: mem::transmute(ptr),
        len: len,
    };
    mem::transmute(raw_slice)
}

#[inline(always)]
pub unsafe fn ptr_to_slice<'a, T>(ptr: *mut T, len: uint) -> &'a [T] {
    let raw_slice : raw::Slice<T> = raw::Slice {
        data: mem::transmute(ptr),
        len: len,
    };
    mem::transmute(raw_slice)
}
