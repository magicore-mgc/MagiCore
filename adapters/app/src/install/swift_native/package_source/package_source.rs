//! Classify resolved Swift artifacts using the existing registry-archive suffix rule.
//! Phân loại artifact Swift theo quy tắc suffix archive registry hiện có.

use mgc_types::ResolvedPackage;

/// Source branch selected for a resolved Swift package.
/// Nhánh source được chọn cho package Swift đã resolve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::install) enum SwiftPackageSource {
    RegistryArchive,
    GitCheckout,
}

/// Preserve the current case-sensitive `.zip` test used by install and pin projection.
/// Giữ predicate `.zip` phân biệt hoa thường đang dùng cho install và pin projection.
pub(in crate::install) fn swift_package_source(package: &ResolvedPackage) -> SwiftPackageSource {
    if package.tarball_url.ends_with(".zip") {
        SwiftPackageSource::RegistryArchive
    } else {
        SwiftPackageSource::GitCheckout
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
