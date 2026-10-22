//! Process-monitor context and compatibility-audit facade.
//! Facade context monitor process và audit compat.

#[path = "compat_audit/mod.rs"]
mod compat_audit;
#[path = "types/mod.rs"]
mod types;

#[cfg(unix)]
#[path = "tracking/mod.rs"]
mod tracking;

pub(in crate::run::implementation) use compat_audit::audit_compat_process;
#[cfg(unix)]
pub(super) use tracking::{PROCESS_TREE_TRACKING_ENV, process_tree_tracking_token};
pub(in crate::run::implementation) use types::{
    CompatAuditContext, NativeAuditContext, ProcessMonitor,
};
