//! Read the selected package declaration from the nearest package manifest.
//! Đọc khai báo package từ manifest gần nhất.

use serde_json::Value;

/// Return the declared range from the first matching dependency section.
/// Trả range đã khai báo tại section dependency khớp đầu tiên.
fn declared_version(manifest: &Value, package: &str) -> Option<String> {
    for section in &[
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ] {
        if let Some(deps) = manifest.get(*section).and_then(|v| v.as_object())
            && let Some(ver) = deps.get(package).and_then(|v| v.as_str())
        {
            return Some(ver.to_string());
        }
    }
    None
}

/// Best-effort lookup for the package's declaration in the current project.
/// Tìm khai báo package trong project hiện tại theo kiểu best-effort.
pub(in crate::commands::info) fn detect_local_version(package: &str) -> Option<String> {
    let cwd = std::env::current_dir().ok()?;
    let root = crate::commands::core::shared::find_project_root(&cwd).ok()??;
    let manifest_path = root.join("package.json");
    if !manifest_path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(manifest_path).ok()?;
    let parsed: Value = serde_json::from_str(&content).ok()?;
    declared_version(&parsed, package)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
