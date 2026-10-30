//! Process-tree monitor facade.
//! Facade monitor cây process.

#[cfg(unix)]
#[path = "error.rs"]
mod error;
#[cfg(any(unix, test))]
#[path = "monitor/mod.rs"]
mod implementation;

#[cfg(unix)]
pub(in crate::run::implementation) use error::MonitoredRootNotVisible;
#[cfg(unix)]
pub(in crate::run::implementation) use implementation::find_forbidden_descendant_tracking;