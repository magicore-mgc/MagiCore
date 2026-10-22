//! Coordinate signed policy inputs for command execution.
//! Điều phối đầu vào policy ký cho lệnh thực thi.

use std::path::Path;

/// Load policy, lane context and lock-bound profile for one operation.
/// Nạp policy, ngữ cảnh lane và profile gắn lock cho một thao tác.
pub fn load_policy_for_run(
    project_root: &Path,
    operation: &str,
) -> (
    Option<mgc_exec::policy_manifest::PolicyManifest>,
    Option<mgc_exec::policy::PolicyContext>,
    Option<mgc_exec::policy::LaneProfile>,
) {
    let manifest = super::manifest_load::load_active_exception_manifest(project_root);
    let context = super::context::lane_context(project_root, operation);
    let profile = context
        .as_ref()
        .and_then(|context| super::context::active_profile(project_root, context));
    (manifest, context, profile)
}
