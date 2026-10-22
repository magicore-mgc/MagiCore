//! Process-monitor and audit context records shared across run owners.
//! Bản ghi monitor và audit context dùng chung giữa các owner run.

#[cfg(windows)]
use super::super::super::ShadowPath;
use crate::allowlist::ExecutionScope;
use std::path::Path;
use std::sync::Mutex;

pub(in crate::run::implementation) struct ProcessMonitor<'a> {
    pub(in crate::run::implementation) enabled: bool,
    pub(in crate::run::implementation) execution_scope: ExecutionScope,
    pub(in crate::run::implementation) exempt: &'a [&'a str],
    pub(in crate::run::implementation) compat_audit: Option<&'a CompatAuditContext<'a>>,
    pub(in crate::run::implementation) native_audit: Option<&'a NativeAuditContext<'a>>,
    #[cfg(unix)]
    pub(in crate::run::implementation) shadow_dir: &'a Path,
    #[cfg(windows)]
    pub(in crate::run::implementation) shadow_path: &'a ShadowPath,
    #[cfg(unix)]
    pub(in crate::run::implementation) tracking_token: &'a str,
}

pub(in crate::run::implementation) struct CompatAuditContext<'a> {
    pub(in crate::run::implementation) manager: &'a str,
    pub(in crate::run::implementation) log_path: &'a Path,
    pub(in crate::run::implementation) cwd: &'a Path,
    pub(in crate::run::implementation) seen_processes:
        Mutex<std::collections::HashSet<(u32, String)>>,
}

pub(in crate::run::implementation) struct NativeAuditContext<'a> {
    pub(in crate::run::implementation) log_path: &'a Path,
    pub(in crate::run::implementation) root_cmd: &'a str,
    pub(in crate::run::implementation) root_args: &'a [String],
    pub(in crate::run::implementation) cwd: &'a Path,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
