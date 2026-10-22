//! `installed_package.rs` — installed package version and identity checks.
//! kiểm tra version và identity package đã cài

use mgc_types::{PackageId, Version};
use std::path::Path;

pub fn installed_package_version(path: &Path) -> Option<Version> {
    let package_json = path.join("package.json");
    let contents = std::fs::read_to_string(package_json).ok()?;
    let value: serde_json::Value = serde_json::from_str(&contents).ok()?;
    let version = value.get("version")?.as_str()?;
    Version::parse(version).ok()
}

pub fn installed_package_matches(path: &Path, package_id: &PackageId) -> bool {
    installed_package_version(path)
        .map(|version| version == *package_id.version())
        .unwrap_or(false)
}
