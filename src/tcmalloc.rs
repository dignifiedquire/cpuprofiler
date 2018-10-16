use std::alloc::{GlobalAlloc, Layout};
use std::os::raw::c_void;

use heap_profiler::{tc_free, tc_memalign};

// based on https://github.com/jmcomets/tcmalloc-rs

pub struct TCMalloc;

unsafe impl GlobalAlloc for TCMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        tc_memalign(layout.align(), layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        tc_free(ptr as *mut c_void);
        // tc_free_sized(ptr as *mut c_void, layout.size());
    }
}
