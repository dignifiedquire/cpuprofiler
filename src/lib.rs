#![warn(missing_debug_implementations)]
#![feature(allocator_api)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

mod state;
mod tcmalloc;

pub mod error;
pub mod heap_profiler;
pub mod profiler;

#[global_allocator]
static GLOBAL: tcmalloc::TCMalloc = tcmalloc::TCMalloc;
