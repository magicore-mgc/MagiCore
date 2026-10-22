//! Owns the default profile-store path and fail-before-spawn gate bound to current lock bytes. — Sở hữu đường dẫn profile store mặc định và gate fail-before-spawn gắn với byte lock hiện hành.

use std::path::Path;

use anyhow::Result;

use super::super::web::resolve_web_framework;

/// Default profile store directory (user/machine scope).
/// Thư mục store profile mặc định (phạm vi user/máy).
pub fn default_profile_store_dir() -> std::path::PathBuf {
    mgc_platform::paths::mgc_data_dir().join("profiles")
}
/// Pre-flight profile gate (Spec rev4 §3.1 row 3): a lane whose active
/// profile observably requires package managers refuses upfront in native
/// mode — fail before spawn, never a mid-run kill. Lanes without a profile
/// pass through (level-2 tripwire-only, enforced at runtime).
/// (Cổng profile pre-flight (Spec rev4 §3.1 hàng 3): lane mà profile hiệu lực
/// cho thấy chắc chắn cần package manager thì native mode từ chối upfront —
/// fail trước spawn, không kill giữa chừng. Lane chưa profile thì qua
/// (mức 2 tripwire-only, thực thi lúc runtime).)
pub fn check_profile_gate(
    project_root: &Path,
    store_dir: &Path,
    operation: &str,
    compat_runner: Option<&str>,
) -> Result<()> {
    let Some((framework, version)) = resolve_web_framework(project_root)? else {
        return Ok(());
    };
    // Lock-bytes digest binds the profile to this exact lock (Spec rev4 §6).
    // (Digest byte lock ràng buộc profile vào đúng lock này (Spec rev4 §6).)
    let lock_bytes = std::fs::read(project_root.join("mgc.lock")).map_err(|e| {
        crate::error::stale_lock_needs_install(&format!("cannot read mgc.lock: {e}"))
    })?;
    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(&lock_bytes).to_hex();
    let profile = match mgc_exec::spawn_profile::find_active_profile(
        store_dir,
        &framework,
        &version,
        std::env::consts::OS,
        operation,
        Some(&digest),
    ) {
        Ok(profile) => profile,
        Err(mgc_exec::spawn_profile::ProfileLookupError::Invalid(detail)) => {
            return Err(crate::error::invalid_spawn_profile(&detail));
        }
        Err(error) => return Err(crate::error::invalid_spawn_profile(&format!("{error:?}"))),
    };
    let Some(profile) = profile else {
        return Ok(());
    };
    if !profile.requires_pm.is_empty()
        && !profile
            .requires_pm
            .iter()
            .all(|required| compat_runner == Some(required.as_str()))
    {
        return Err(crate::error::lane_needs_compat_runner(
            &framework,
            &profile.requires_pm,
        ));
    }
    Ok(())
}
