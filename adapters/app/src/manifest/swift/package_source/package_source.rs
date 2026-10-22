//! Reads Package.swift safely and accepts only supported literal declarations.
//! Đọc Package.swift an toàn và chỉ nhận khai báo literal được hỗ trợ.

use mgc_adapter_base::project_file::read_regular_text;
use mgc_resolver::protocols::swift::{SwiftDep, parse_swift_package_deps};
use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

/// Confirm the manifest exists before parsing, preserving the established error boundary.
/// Xác nhận manifest tồn tại trước khi parse, giữ nguyên ranh giới lỗi hiện hữu.
pub(in crate::manifest::swift) fn manifest_path(project_root: &Path) -> MgResult<PathBuf> {
    let swift_path = project_root.join("Package.swift");
    match std::fs::symlink_metadata(&swift_path) {
        Ok(_) => Ok(swift_path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err(MgError::Other("Package.swift not found".to_string()))
        }
        Err(error) => Err(MgError::Other(format!("inspect Package.swift: {error}"))),
    }
}

/// Read the regular manifest and reject dynamic or unsupported dependency shapes.
/// Đọc manifest thường và từ chối shape dependency động hoặc không hỗ trợ.
pub(in crate::manifest::swift) fn read_literal_dependencies(
    path: &Path,
) -> MgResult<Vec<SwiftDep>> {
    let source = read_regular_text(path, "Package.swift")?;
    if !source.contains("Package(") {
        return Err(unsupported_static_swift_manifest(
            "no literal Package(...) declaration was found",
        ));
    }
    let package_calls = source.matches(".package(").count();
    let deps = parse_swift_package_deps(&source);
    if deps.len() != package_calls {
        return Err(unsupported_static_swift_manifest(
            "one or more .package(...) declarations use unsupported or dynamic syntax",
        ));
    }
    if deps.iter().any(|dep| dep.requirement_text() == "*") {
        return Err(unsupported_static_swift_manifest(
            "one or more .package(...) requirements are not recognized as literal versions",
        ));
    }
    if let Some(value) = source
        .split_once("dependencies:")
        .map(|(_, value)| value.trim_start())
        && !value.starts_with('[')
    {
        return Err(unsupported_static_swift_manifest(
            "the dependencies argument is computed instead of a literal array",
        ));
    }
    Ok(deps)
}

/// Build the existing fail-closed diagnostic for unsupported Swift source shapes.
/// Tạo diagnostic fail-closed hiện có cho shape source Swift chưa hỗ trợ.
pub(in crate::manifest::swift) fn unsupported_static_swift_manifest(reason: &str) -> MgError {
    MgError::Unsupported {
        core: "app",
        capability: "swift_manifest_parse",
        guidance: format!(
            "Package.swift is executable Swift; MagiCore parses only literal .package(...) declarations without running SwiftPM. {reason}. Use a supported literal manifest shape or manage this project with its Swift toolchain outside MagiCore."
        ),
    }
}

/// Scan Package.swift text without requiring an installed Swift toolchain.
/// Quét text Package.swift mà không cần cài Swift toolchain.
pub fn scan_package_swift_text(text: &str) -> Vec<SwiftDep> {
    parse_swift_package_deps(text)
}
