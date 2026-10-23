//! Package identity and dependency-edge marker validation.
//! Định danh package và validate marker dependency-edge.

use mgc_lockfile::Package;

/// Chấp nhận marker chỉ khi dependency vẫn nằm trong Cargo lock closure:
/// optional crate hợp lệ hoặc target cfg không rỗng.
pub(crate) fn is_cargo_dependency_edge_marker(marker: &str) -> bool {
    if let Some(name) = marker.strip_prefix("optional:") {
        return !name.is_empty() && mgc_types::PackageName::new(name.to_string()).is_ok();
    }
    marker
        .strip_prefix("target:")
        .is_some_and(|target| !target.trim().is_empty())
}

pub(in crate::install) fn package_id(package: &Package) -> String {
    format!("{}@{}", package.name, package.version)
}
