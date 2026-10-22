use super::super::super::super::*;

/// Build is a MagiCore-owned route; historical rival-runtime flags are refused.
/// (Build thuộc luồng MagiCore; cờ runtime đối thủ cũ luôn bị từ chối.)
pub(in crate::commands::build::implementation) fn ensure_native_build_mode(
    mode: &crate::commands::compat::CompatMode,
) -> Result<()> {
    match mode {
        crate::commands::compat::CompatMode::Native => Ok(()),
        crate::commands::compat::CompatMode::Explicit(runtime) => {
            Err(crate::error::rival_runtime_not_native(runtime))
        }
    }
}

/// Keep the legacy Cargo build lane only for a plain, unmarked Rust project.
/// Chỉ dùng nhánh Cargo legacy cho Rust project thuần, chưa có identity MagiCore.
pub(in crate::commands::build::implementation) fn should_use_legacy_rust_build(
    root: &Path,
) -> Result<bool> {
    let detected = mgc_config::project::ProjectConfig::detect_core(root)?;
    if detected.as_deref() != Some("lib") {
        return Ok(false);
    }
    let has_config =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")?
            .is_some();
    let has_marker = mgc_config::project::ProjectConfig::read_core_marker(root)?.is_some();
    Ok(!has_config && !has_marker)
}

/// A build backend selector accepts only regular project manifests.
/// Chỉ cho bộ chọn backend nhận manifest là file thường, không theo symlink.
pub(in crate::commands::build::implementation) fn project_manifest_present(
    root: &Path,
    name: &str,
) -> Result<bool> {
    Ok(mgc_config::project::read_regular_project_text(&root.join(name), name)?.is_some())
}

pub(in crate::commands::build::implementation) fn find_root() -> anyhow::Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    if let Some(root) = mgc_config::project::ProjectConfig::find_project_root(&cwd) {
        return Ok(root);
    }
    Err(crate::error::no_project_found_build())
}
