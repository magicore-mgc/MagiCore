//! Validates project core marker/config identity before store operations.
//! Xác thực marker/cấu hình core trước thao tác store.

use anyhow::Result;
use mgc_config::project::ProjectConfig;
use std::path::Path;

/// The current store subcommands operate on the Web store layout only.
/// Require a consistent project signature/config before status, prune,
/// backup, restore, or doctor can touch that core-owned data.
pub(in crate::commands::store) fn ensure_web_store_owner(project_root: &Path) -> Result<()> {
    let marker = ProjectConfig::read_core_marker(project_root)?
        .ok_or_else(crate::error::store_core_signature_required)?;
    let configured = ProjectConfig::load(project_root)?.map(|config| {
        if config.ecosystem == "cloud" {
            "clo".to_string()
        } else {
            config.ecosystem
        }
    });

    if let Some(configured) = &configured
        && &marker != configured
    {
        return Err(crate::error::store_core_identity_conflict(
            &marker, configured,
        ));
    }
    if marker != "web" {
        return Err(crate::error::store_requires_web_core(Some(&marker)));
    }
    Ok(())
}
/// Check the project core marker before touching a non-Web shared store.
/// Đối chiếu marker core trước khi kiểm tra shared store ngoài Web.
pub(in crate::commands::store) fn ensure_store_core_owner(
    project_root: &Path,
    expected: &str,
) -> Result<()> {
    let marker = ProjectConfig::read_core_marker(project_root)?
        .ok_or_else(|| crate::error::store_core_marker_required(expected))?;
    let configured = ProjectConfig::load(project_root)?.map(|config| {
        if config.ecosystem == "cloud" {
            "clo".to_string()
        } else {
            config.ecosystem
        }
    });
    if let Some(configured) = &configured
        && &marker != configured
    {
        return Err(crate::error::store_core_identity_conflict_any(
            &marker, configured,
        ));
    }
    if marker != expected {
        return Err(crate::error::store_core_identity_mismatch(
            expected,
            &marker,
            configured.as_deref().unwrap_or("<missing mgc.toml core>"),
        ));
    }
    Ok(())
}

/// Refuse a caller-selected framework that disagrees with mgc.toml.
/// Từ chối framework caller chọn nếu không khớp mgc.toml.
pub(in crate::commands::store) fn ensure_store_framework_owner(
    project_root: &Path,
    core: &str,
    expected: &str,
) -> Result<()> {
    ensure_store_core_owner(project_root, core)?;
    let config = ProjectConfig::load(project_root)?
        .ok_or_else(|| crate::error::store_doctor_framework_metadata_required(core, expected))?;
    if !config
        .frameworks
        .iter()
        .any(|framework| store_framework_name_matches(core, framework, expected))
    {
        return Err(crate::error::store_doctor_framework_mismatch(
            core,
            expected,
            &config.frameworks,
        ));
    }
    Ok(())
}

/// Match the small set of Web aliases accepted by the store route against
/// the canonical names written to `mgc.toml` by scaffold normalization.
/// So khớp alias Web được store route nhận với tên chuẩn scaffold ghi vào `mgc.toml`.
pub(in crate::commands::store) fn store_framework_name_matches(
    core: &str,
    declared: &str,
    requested: &str,
) -> bool {
    fn canonical<'a>(core: &str, framework: &'a str) -> &'a str {
        match (core, framework) {
            ("web", "ts") => "vanilla",
            ("web", "node") => "express",
            _ => framework,
        }
    }
    canonical(core, declared) == canonical(core, requested)
}
