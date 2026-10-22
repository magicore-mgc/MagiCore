//! Data retained between preflight checks and spawn orchestration.
//! Dữ liệu giữ lại giữa bước preflight và orchestration spawn.

use std::{path::Path, time::Duration};

/// Validated, redacted command context ready for either dry-run or live setup.
/// Ngữ cảnh command đã xác thực, redact, sẵn sàng dry-run hoặc setup thật.
pub(in crate::run::implementation) struct PreparedInvocation<'a> {
    pub(in crate::run::implementation) safe_args: Vec<String>,
    pub(in crate::run::implementation) cwd: std::path::PathBuf,
    pub(in crate::run::implementation) timeout: Option<Duration>,
    pub(in crate::run::implementation) compat_manager: Option<String>,
    pub(in crate::run::implementation) compat_log_path: Option<&'a Path>,
}
