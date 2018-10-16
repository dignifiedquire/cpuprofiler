//! Heap Profiler
//!
//! # Usage
//!
//! ```
//! use cpuprofiler::heap_profiler::HEAP_PROFILER;
//!
//! HEAP_PROFILER.lock().unwrap().start("./my-heap-prof.profile");
//! // Code you want to sample goes here!
//! HEAP_PROFILER.lock().unwrap().stop();
//! ```
//!
//! The profiler is accessed via the static `HEAP_PROFILER: Mutex<HeapProfiler>`.
//! We limit access this way to ensure that only one profiler is running at a time -
//! this is a limitation of the heap-profiler library.

use std::ffi::CString;
use std::fs::File;
use std::os::raw::c_char;
use std::path::Path;

use error::{Error, ErrorKind};
use state::ProfilerState;

use std::sync::Mutex;

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

#[link(name = "tcmalloc")]
#[allow(non_snake_case)]
extern "C" {
    fn HeapProfilerStart(fname: *const c_char);

    fn HeapProfilerStop();

    fn HeapProfilerDump(resaon: *const c_char);
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
    /// Returns the profiler state
    ///
    /// # Examples
    ///
    /// ```
    /// use cpuprofiler::heap_profiler::HEAP_PROFILER;
    ///
    /// println!("{}", HEAP_PROFILER.lock().unwrap().state());
    /// ```
    pub fn state(&self) -> ProfilerState {
        self.state
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
    /// - An internal failure from the cpuprofiler library.
    pub fn start<T: Into<Vec<u8>>>(&mut self, fname: T) -> Result<(), Error> {
        if self.state == ProfilerState::NotActive {
            let c_fname = try!(CString::new(fname));
            try!(check_file_path(c_fname.clone().into_string().unwrap()));
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

    pub fn dump<T: Into<Vec<u8>>>(&mut self, reason: T) -> Result<(), Error> {
        let c_reason = try!(CString::new(reason));
        try!(check_file_path(c_reason.clone().into_string().unwrap()));
        unsafe {
            HeapProfilerDump(c_reason.as_ptr());
        }
        Ok(())
    }
}

fn check_file_path<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let write_res = File::create(path);

    match write_res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
