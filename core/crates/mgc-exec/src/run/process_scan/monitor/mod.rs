//! Process-tree monitor facade.
//! Facade monitor cây process.

#[cfg(unix)]
#[path = "error.rs"]
mod error;
// The Linux snapshot tests reach the identity helper through this module, so it
// is visible at the scope they share with the probe.
// (Test snapshot Linux chạm helper identity qua module này, nên nó visible ở
// scope mà test dùng chung với probe.)
#[cfg(any(unix, test))]
#[path = "monitor/mod.rs"]
pub(in crate::run::implementation) mod implementation;

#[cfg(unix)]
pub(in crate::run::implementation) use error::MonitoredRootNotVisible;
#[cfg(unix)]
pub(in crate::run::implementation) use implementation::find_forbidden_descendant_tracking;
