use std::alloc::{self, Layout};
pub use witx_bindgen_rust_impl::{export, import};

#[no_mangle]
unsafe extern "C" fn witx_malloc(len: usize, align: usize) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(len, align);
    let ptr = alloc::alloc(layout);
    if ptr.is_null() {
        alloc::handle_alloc_error(layout);
    }
    return ptr;
}

#[no_mangle]
unsafe extern "C" fn witx_free(ptr: *mut u8, len: usize, align: usize) {
    let layout = Layout::from_size_align_unchecked(len, align);
    alloc::dealloc(ptr, layout);
}

/// A trait for handle types which are wrappers around `u32` indices.
pub unsafe trait HandleIndex {
    /// Wrap a `u32` in a `Self`.
    unsafe fn from_raw(raw: u32) -> Self;

    /// Consume `self` and return the contained `u32`.
    fn into_raw(self) -> u32;

    /// Return the contained `u32`.
    fn as_raw(&self) -> u32;
}

pub mod exports;
pub mod imports;
