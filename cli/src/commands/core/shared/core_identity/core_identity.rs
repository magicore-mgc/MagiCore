use super::*;

#[allow(dead_code)]
pub fn find_project_root(cwd: &Path) -> Result<Option<PathBuf>> {
    Ok(mgc_config::project::ProjectConfig::find_project_root(cwd))
}

/// Refuse a core command when the persisted/detected project identity belongs
/// to another core, even if an adjacent manifest matches this command's lane.
/// Từ chối lệnh nếu identity project thuộc core khác dù có manifest trùng.
pub(crate) fn ensure_project_core_identity(root: &Path, expected: &str) -> Result<()> {
    let detected = mgc_config::project::ProjectConfig::detect_core(root)?;
    let Some(detected) = detected else {
        return Err(crate::error::project_core_identity_missing(root, expected));
    };
    // Both sides go through the ONE alias table: `mgc_config` already returns
    // canonical names, and `expected` may still arrive as the `cloud` host tag
    // from an adapter-facing caller.
    // (Hai phía đều đi qua bảng alias duy nhất; `expected` có thể vẫn là tag
    // host `cloud` từ caller phía adapter.)
    let expected = mgc_types::Ecosystem::canonical_core_name(expected);
    let detected = mgc_types::Ecosystem::canonical_core_name(&detected);
    if detected != expected {
        return Err(crate::error::project_core_identity_mismatch(
            root, &expected, &detected,
        ));
    }
    Ok(())
}

/// project_root của core — seeded từ find_project_root (mgc.toml/.mgc.core/package.json...).
pub fn core_project_root(core: &str) -> Result<PathBuf> {
    let cwd = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
    let root = find_project_root(&cwd)?.ok_or_else(|| crate::error::no_mgc_project_found(core))?;
    ensure_project_core_identity(&root, core)?;
    Ok(root)
}
