use std::fmt;

/// The state of the profiler
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfilerState {
    /// When the profiler is active
    Active,
    /// When the profiler is inactive
    NotActive,
}

impl fmt::Display for ProfilerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ProfilerState::Active => write!(f, "Active"),
            ProfilerState::NotActive => write!(f, "NotActive"),
        }
    }
}
