#![warn(missing_debug_implementations)]
#![cfg_attr(feature = "heap", feature(allocator_api))]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

mod state;
#[cfg(feature = "heap")]
mod tcmalloc;

pub mod error;
pub mod profiler;

#[cfg(feature = "heap")]
pub mod heap_profiler;

#[cfg(feature = "heap")]
#[global_allocator]
static GLOBAL: tcmalloc::TCMalloc = tcmalloc::TCMalloc;
