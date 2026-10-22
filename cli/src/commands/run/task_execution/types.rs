//! Private request exchanged within task execution.
//! Request riêng dùng bên trong owner thực thi task.

/// One script execution request — bundling keeps the arg list reviewable.
/// (Một yêu cầu thực thi script — gom lại để danh sách tham số dễ review.)
use std::path::Path;
use std::path::PathBuf;
#[allow(clippy::too_many_arguments)]
pub(in crate::commands::run) struct TaskRequest<'a> {
    pub(super) cmd: &'a str,
    pub(super) args: &'a [String],
    pub(super) cwd: &'a Path,
    pub(super) script_name: &'a str,
    pub(super) bin_path: Option<PathBuf>,
    pub(super) compat: &'a crate::commands::compat::CompatMode,
    pub(super) compat_runner: Option<&'a str>,
    pub(super) policy_manifest: Option<mgc_exec::policy_manifest::PolicyManifest>,
    pub(super) policy_context: Option<mgc_exec::policy::PolicyContext>,
    pub(super) policy_profile: Option<mgc_exec::policy::LaneProfile>,
}
