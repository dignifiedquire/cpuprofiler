use std::alloc::{GlobalAlloc, Layout};
use std::os::raw::c_void;

#[allow(non_snake_case)]
extern "C" {
    fn tc_memalign(alignment: usize, size: usize) -> *mut c_void;
    // fn tc_free(ptr: *mut c_void);
    fn tc_free_sized(ptr: *mut c_void, size: usize);
}

pub struct TCMalloc;

unsafe impl GlobalAlloc for TCMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        tc_memalign(layout.align(), layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        tc_free_sized(ptr as *mut c_void, layout.size());
    }
}
