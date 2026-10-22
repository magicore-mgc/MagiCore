//! Resolves default and named catalog specifiers.
//! Phân giải specifier catalog mặc định và có tên.

use super::super::WorkspaceCatalogs;

/// Resolve a catalog: specifier to its configured version.
/// Phân giải specifier catalog: thành version đã cấu hình.
pub fn resolve_catalog_specifier(
    package_name: &str,
    specifier: &str,
    catalogs: &WorkspaceCatalogs,
) -> Option<String> {
    let catalog_name = specifier.strip_prefix("catalog:")?.trim();
    if catalog_name.is_empty() || catalog_name == "default" {
        catalogs.default.get(package_name).cloned()
    } else {
        catalogs
            .named
            .get(catalog_name)
            .and_then(|cat| cat.get(package_name))
            .cloned()
    }
}
