#![warn(missing_debug_implementations)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

mod state;
mod util;

pub mod error;
pub mod profiler;

pub use profiler::*;

#[cfg(feature = "heap")]
pub mod heap_profiler;
#[cfg(feature = "heap")]
pub use heap_profiler::*;
#[cfg(feature = "heap")]
mod tcmalloc;
#[cfg(feature = "heap")]
static GLOBAL: tcmalloc::TCMalloc = tcmalloc::TCMalloc;
