use core::{alloc::GlobalAlloc, ptr::null_mut};

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        panic!("dealloc should be never called")
    }
}