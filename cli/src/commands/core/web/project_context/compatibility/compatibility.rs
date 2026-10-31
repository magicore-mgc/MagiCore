// Web freshness and package-manager compatibility gates — kiểm tra độ mới và cổng tương thích.
// Web freshness and package-manager compatibility.
use super::*;

/// Pre-flight freshness: every dependency declared in `package.json`
/// must have a locked entry in `mgc.lock` (Spec rev4 §8).
/// Only dependency metadata participates — editing `scripts.*` alone never
/// triggers a resolve. Missing/invalid lock fails closed.
/// (Freshness pre-flight: mọi dependency khai báo trong `package.json` phải
/// có entry trong `mgc.lock` (Spec rev4 §8). Chỉ metadata dependency được so —
/// sửa `scripts.*` không trigger resolve. Thiếu/sai lock thì fail-closed.)
pub fn check_dev_freshness(project_root: &Path) -> Result<()> {
    let manifest_path = project_root.join("package.json");
    if !manifest_path.exists() {
        return Ok(());
    }
    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path)
            .with_context(|| format!("cannot read {}", manifest_path.display()))?,
    )
    .with_context(|| format!("{} is not valid JSON", manifest_path.display()))?;

    let declared = crate::commands::freshness::declared_npm_dep_specs(&manifest)?;
    crate::commands::freshness::check_dev_npm_freshness_against_lock(project_root, &declared)
}
pub(in crate::commands::core::web::implementation) fn non_native_member_ecosystem(
    project_root: &Path,
) -> Option<&'static str> {
    if project_root.join("go.mod").exists() {
        Some("go")
    } else if project_root.join("requirements.txt").exists() {
        Some("python")
    } else if project_root.join("Cargo.toml").exists() {
        Some("rust")
    } else if project_root.join("pom.xml").exists() {
        Some("java")
    } else if project_root.join("composer.json").exists() || project_root.join("artisan").exists() {
        Some("php")
    } else {
        None
    }
}
/// Fail closed for non-JS package manifests until MGC owns their lifecycle.
/// Từ chối manifest ngoài JS đến khi MGC sở hữu lifecycle tương ứng.
pub(in crate::commands::core::web::implementation) fn compat_gate_decision(
    project_root: &Path,
) -> Result<&'static str> {
    let Some(ecosystem) = non_native_member_ecosystem(project_root) else {
        return Err(crate::error::web_no_install_flow(project_root));
    };
    Err(crate::error::native_dependency_engine_unavailable(
        "web monorepo member",
        ecosystem,
        "install",
    ))
}
pub(in crate::commands::core::web::implementation) fn compat_install_target(
    project_root: &Path,
    _compat: &crate::commands::compat::CompatMode,
) -> Result<()> {
    compat_gate_decision(project_root).map(|_| ())
}

/// Whether an install target is a server half MagiCore starts itself.
/// Target install có phải nửa server MagiCore tự khởi động hay không.
///
/// `mgc dev` starts such a half with its own runner, so there is no package graph
/// for this installer to resolve. The predicate is deliberately narrower than
/// "has a dependency manifest": it requires the entry file the runner executes,
/// which keeps the fail-closed decision for a manifest-only member intact.
/// `mgc dev` khởi động nửa đó bằng runner riêng nên installer này không có graph
/// package để resolve. Predicate cố tình hẹp hơn "có dependency manifest": nó đòi
/// đúng file entry mà runner chạy, giữ nguyên quyết định fail-closed cho member chỉ
/// có manifest.
pub(in crate::commands::core::web::implementation) fn is_startable_server_half(
    project_root: &Path,
) -> bool {
    // RULE §12: the entry list is data, not inline literals scattered in logic.
    // (RULE §12: danh sách entry là dữ liệu, không rải literal trong logic.)
    const SERVER_ENTRY_FILES: &[&str] = &[
        "main.go",
        "src/main.rs",
        "main.py",
        "app.py",
        "manage.py",
        "src/main.py",
    ];
    !project_root.join("package.json").is_file()
        && SERVER_ENTRY_FILES
            .iter()
            .any(|entry| project_root.join(entry).is_file())
}
