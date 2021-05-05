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

/// A trait for handle types which are wrappers around `i32` indices.
pub unsafe trait HandleIndex {
    /// Wrap an `i32` in a `Self`.
    unsafe fn from_raw(raw: i32) -> Self;

    /// Consume `self` and return the contained `i32`.
    fn into_raw(self) -> i32;

    /// Return the contained `i32`.
    fn as_raw(&self) -> i32;
}

pub mod exports;
pub mod imports;
