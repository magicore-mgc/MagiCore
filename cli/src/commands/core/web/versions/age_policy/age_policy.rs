//! Load the active project/store policy without weakening registry checks.
//! Đọc policy project/store đang áp dụng mà không nới kiểm tra registry.

use super::super::{ScaffoldAgePolicy, error};
use anyhow::Result;
use std::path::Path;

/// Resolve the same active Web release-age policy used by installation.
/// Lấy age policy Web đang dùng lúc install.
#[cfg(feature = "web")]
pub(crate) fn scaffold_age_policy(project_root: &Path) -> Result<Option<ScaffoldAgePolicy>> {
    // The documented escape hatch is checked before anything is parsed, so it
    // also covers a malformed project file — an operator overriding the gate
    // must not be blocked by the file they are overriding from.
    // (Escape hatch tài liệu hóa kiểm trước khi parse, để cũng phủ file hỏng.)
    if env_truthy("MAGICORE_ALLOW_UNTRUSTED") {
        warn_untrusted_override_once();
        return Ok(None);
    }
    let security = mgc_config::project::load_security_config(project_root)
        .map_err(|error| error::web_scaffold_security_config_unreadable(project_root, &error))?;
    let allow_missing_time = security
        .as_ref()
        .and_then(|config| config.allow_missing_time)
        .unwrap_or(false);
    if let Some(cutoff_hours) = security
        .as_ref()
        .and_then(|config| config.min_age_for_ecosystem("web"))
    {
        return Ok(Some(ScaffoldAgePolicy {
            cutoff_hours,
            allow_missing_time,
        }));
    }
    if let Some(min_age_secs) = store_release_policy(project_root)? {
        return Ok(Some(ScaffoldAgePolicy {
            cutoff_hours: min_age_secs.div_ceil(60 * 60),
            allow_missing_time,
        }));
    }
    if env_truthy("MAGICORE_SECURITY_24H_BLOCK") || env_truthy("MGC_AUDIT_STRICT") {
        return Ok(Some(ScaffoldAgePolicy {
            cutoff_hours: mgc_config::DEFAULT_MIN_RELEASE_AGE_HOURS,
            allow_missing_time,
        }));
    }
    // No explicit policy: the documented default applies, so `mgc versions`
    // keeps selecting exactly what `mgc install` will accept. An explicit
    // `0` in the project already returned above as disabled.
    // (Không policy tường minh: áp mặc định để `mgc versions` chọn đúng thứ
    // `mgc install` chấp nhận. Opt-out `0` tường minh đã trả ở trên.)
    Ok(Some(ScaffoldAgePolicy {
        cutoff_hours: mgc_config::DEFAULT_MIN_RELEASE_AGE_HOURS,
        allow_missing_time,
    }))
}

/// Keep optional-Web builds independent of the Web adapter dependency.
/// Giữ build tắt Web không phụ thuộc adapter Web optional.
#[cfg(not(feature = "web"))]
pub(crate) fn scaffold_age_policy(_project_root: &Path) -> Result<Option<ScaffoldAgePolicy>> {
    Ok(None)
}

/// Read the persisted release-age policy only from a regular store database.
/// Chỉ đọc release-age policy từ database store là file thường.
#[cfg(feature = "web")]
pub(in crate::commands::core::web::implementation) fn store_release_policy(
    project_root: &Path,
) -> Result<Option<u64>> {
    let layout = mgc_store::Layout::new(mgc_web_adapter::lockfile::project_cache_dir(project_root));
    let database_path = layout.db_path();
    match std::fs::symlink_metadata(&database_path) {
        Ok(metadata) if metadata.file_type().is_file() => {}
        Ok(_) => {
            return Err(error::web_scaffold_store_policy_path_invalid(
                &database_path,
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error::web_scaffold_store_policy_unreadable(
                &database_path,
                &error,
            ));
        }
    }
    let database = mgc_store::Database::open(&database_path)
        .map_err(|error| error::web_scaffold_store_policy_unreadable(&database_path, &error))?;
    database
        .release_policy("web")
        .map_err(|error| error::web_scaffold_store_policy_unreadable(&database_path, &error))
}

#[cfg(feature = "web")]
fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
}

#[cfg(feature = "web")]
const UNTRUSTED_OVERRIDE_WARNING: &str = "WARNING: Web scaffold release-age checks are disabled by MAGICORE_ALLOW_UNTRUSTED; package versions still resolve from the configured registry.";

#[cfg(feature = "web")]
fn warn_untrusted_override_once() {
    static WARNED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    WARNED.get_or_init(|| eprintln!("{UNTRUSTED_OVERRIDE_WARNING}"));
}

#[cfg(test)]
#[path = "test/age_policy.rs"]
mod tests;
