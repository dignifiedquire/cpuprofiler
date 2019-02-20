//! Heap Profiler
//!
//!
//! # Usage
//!
//! ```
//! use gperftools::HEAP_PROFILER;
//!
//! // Start profiling
//! HEAP_PROFILER.lock().unwrap().start("./my-profile.mprof");
//!
//! {
//!   // do some work
//!   let v = vec![1; 1000];
//!   println!("{:?}", v);
//!   // sleep a bit so we have time to profile
//!   std::thread::sleep_ms(1000);
//! }
//!
//! // stop profiling
//! HEAP_PROFILER.lock().unwrap().stop();
//! ```
//!
//! The following environment flags can change the behaviour of the profiler.
//!
//! - `HEAP_PROFILE_ALLOCATION_INTERVAL` (Default: 1GB) - If non-zero, dump heap profiling information once every specified number of bytes allocated by the program since the last dump.
//! - `HEAP_PROFILE_DEALLOCATION_INTERVAL` (Default: 0) - If non-zero, dump heap profiling information once every specified number of bytes deallocated by the program since the last dump.
//! - `HEAP_PROFILE_INUSE_INTERVAL` (Default: 100MB) - If non-zero, dump heap profiling information whenever the high-water memory usage mark increases by the specified number of bytes.
//! - `HEAP_PROFILE_TIME_INTERVAL` (Default: 0) - If non-zero, dump heap profiling information once every specified number of seconds since the last dump.
//! - `HEAP_PROFILE_MMAP_LOG` (Default: false) - Should mmap/munmap calls be logged?
//! - `HEAP_PROFILE_MMAP` (Default: false) - If heap-profiling is on, also profile mmap, mremap, and sbrk
//! - `HEAP_PROFILE_ONLY_MMAP` (Default: false) - If heap-profiling is on, only profile mmap, mremap, and sbrk; do not profile malloc/new/etc
//!
//! The profiler is accessed via the static `HEAP_PROFILER: Mutex<HeapProfiler>`.
//! We limit access this way to ensure that only one profiler is running at a time -
//! this is a limitation of the heap-profiler library.

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;

use error::{Error, ErrorKind};
use state::ProfilerState;
use util::check_file_path;

lazy_static! {
    /// Static reference to the HEAP_PROFILER
    ///
    /// The heap-rofiler library only supports one active profiler.
    /// Because of this we must use static access and wrap in a `Mutex`.
    #[derive(Debug)]
    pub static ref HEAP_PROFILER: Mutex<HeapProfiler> = Mutex::new(HeapProfiler {
        state: ProfilerState::NotActive,
    });
}

#[allow(non_snake_case)]
extern "C" {
    fn HeapProfilerStart(fname: *const c_char);

    fn HeapProfilerStop();

    fn HeapProfilerDump(resaon: *const c_char);

    fn IsHeapProfilerRunning() -> c_int;
}

/// The `HeapProfiler`
///
/// The `HeapProfiler` gives access to the _heap-profiler_ library.
/// By storing the state of the profiler and limiting access
/// we make the FFI safer.
#[derive(Debug)]
pub struct HeapProfiler {
    state: ProfilerState,
}

impl HeapProfiler {
    /// Returns the profiler state.
    ///
    /// # Examples
    ///
    /// ```
    /// use gperftools::heap_profiler::HEAP_PROFILER;
    ///
    /// println!("{}", HEAP_PROFILER.lock().unwrap().state());
    /// ```
    pub fn state(&self) -> ProfilerState {
        self.state
    }

    /// Checks if the heap profiler is running.
    ///
    /// # Examples
    ///
    /// ```
    /// use gperftools::heap_profiler::HEAP_PROFILER;
    ///
    /// println!("running? {}", HEAP_PROFILER.lock().unwrap().is_running());
    /// ```
    pub fn is_running(&self) -> bool {
        let state = unsafe { IsHeapProfilerRunning() };

        state == 1
    }

    /// Start the heap profiler
    ///
    /// Will begin sampling once this function has been called
    /// and will not stop until the `stop` function has been called.
    ///
    /// This function takes as an argument a filename. The filename must be
    /// both valid Utf8 and a valid `CString`.
    ///
    /// # Failures
    ///
    /// - The profiler is currently `Active`.
    /// - `fname` is not a valid `CString`.
    /// - `fname` is not valid Utf8.
    /// - `fname` is not a file.
    /// - The user does not have write access to the file.
    /// - An internal failure from the gperftools library.
    pub fn start<T: Into<Vec<u8>>>(&mut self, fname: T) -> Result<(), Error> {
        if self.state == ProfilerState::NotActive {
            let c_fname = try!(CString::new(fname));
            check_file_path(c_fname.clone().into_string().unwrap())?;
            unsafe {
                HeapProfilerStart(c_fname.as_ptr());
            }
            self.state = ProfilerState::Active;
            Ok(())
        } else {
            Err(ErrorKind::InvalidState(self.state).into())
        }
    }

    /// Stop the heap profiler.
    ///
    /// This will stop the profiler if it `Active` and return
    /// an error otherwise.
    ///
    /// # Failures
    ///
    /// - The profiler is `NotActive`.
    pub fn stop(&mut self) -> Result<(), Error> {
        if self.state == ProfilerState::Active {
            unsafe {
                HeapProfilerStop();
            }
            self.state = ProfilerState::NotActive;
            Ok(())
        } else {
            Err(ErrorKind::InvalidState(self.state).into())
        }
    }

    /// Manually trigger a dump of the current profile.
    pub fn dump<T: Into<Vec<u8>>>(&mut self, reason: T) -> Result<(), Error> {
        let c_reason = try!(CString::new(reason));
        check_file_path(c_reason.clone().into_string().unwrap())?;
        unsafe {
            HeapProfilerDump(c_reason.as_ptr());
        }
        Ok(())
    }
}
