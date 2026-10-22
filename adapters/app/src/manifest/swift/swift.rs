//! Coordinates safe static Swift manifest parsing and public App manifest behavior.
//! Điều phối parse manifest Swift tĩnh an toàn và API manifest App.

use mgc_types::{
    DependencySpec, Ecosystem, Manifest, MgError, MgResult, PackageName, VersionRange,
};
use std::path::Path;

use super::package_source::{
    manifest_path, read_literal_dependencies, unsupported_static_swift_manifest,
};
use super::resolved_pins::{pin_range_for, read_pins};

/// Parse only literal `.package(...)` declarations without running SwiftPM.
/// Chỉ parse khai báo `.package(...)` literal mà không chạy SwiftPM.
pub fn parse_package_swift(project_root: &Path) -> MgResult<Manifest> {
    let swift_path = manifest_path(project_root)?;
    let name = project_root
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "app".to_string());
    let mut manifest = Manifest::new(&name, Ecosystem::App);
    let deps = read_literal_dependencies(&swift_path)?;
    let pins = read_pins(project_root)?;

    for dep in deps {
        let dep_name = dep.dep_name();
        let range = pin_range_for(&dep, &pins).unwrap_or_else(|| dep.requirement_text());
        let dep_name = PackageName::new(dep_name).map_err(|e| {
            unsupported_static_swift_manifest(&format!("dependency identity is unsupported: {e}"))
        })?;
        let range = VersionRange::parse(&range).map_err(|e| {
            unsupported_static_swift_manifest(&format!(
                "dependency version range is unsupported: {e}"
            ))
        })?;
        manifest.add_dep(DependencySpec::new(dep_name, range), false, false, false);
    }

    Ok(manifest)
}

/// Refuse to rewrite executable Swift source instead of reporting a false success.
/// Từ chối sửa mã nguồn Swift thực thi thay vì báo thành công giả.
pub fn write_package_swift(_project_root: &Path, _manifest: &Manifest) -> MgResult<()> {
    Err(MgError::Unsupported {
        core: "app",
        capability: "write-swift-manifest",
        guidance: "MagiCore does not rewrite executable Package.swift source; Swift dependency mutation is unsupported by this writer".to_string(),
    })
}
