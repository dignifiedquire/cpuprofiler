#![warn(missing_debug_implementations)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

// we need to use a custom allocator to enable heap profiling
extern crate tcmalloc;

use tcmalloc::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

mod state;

pub mod error;
pub mod heap_profiler;
pub mod profiler;
