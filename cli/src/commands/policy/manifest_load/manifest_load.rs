//! Select the active signed exception manifest.
//! Chọn manifest exception có ký đang hiệu lực.

use std::path::Path;

/// Load the active exception manifest for a project, if any verifies.
/// Returns `None` when no candidate verifies — callers keep native-deny.
/// (Nạp manifest exception hiệu lực cho project, nếu có bản nào xác minh
/// được. Trả `None` khi không bản nào xác minh được — caller giữ native-deny.)
pub fn load_active_exception_manifest(
    project_root: &Path,
) -> Option<mgc_exec::policy_manifest::PolicyManifest> {
    let trusted = super::trust_keys::trusted_policy_key_ids();
    if trusted.is_empty() {
        return None;
    }
    let now = mgc_exec::policy_manifest::now_unix_secs();
    mgc_exec::policy_manifest::select_active_manifest(
        super::manifest_candidates::read_candidates(project_root),
        &trusted,
        &[],
        now,
    )
}
