//! Preserve exact identity for one direct resolver request.
//! Giữ chính xác định danh của một request resolver trực tiếp.

use mgc_types::{PackageId, PackageName};

/// Exact selected package for one request from the project's root manifest.
/// Package được chọn chính xác cho một request từ manifest root của project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootResolution {
    pub request_index: usize,
    pub request_name: PackageName,
    pub request_spec: String,
    pub package_id: PackageId,
}

impl RootResolution {
    pub(crate) fn new(
        request_index: usize,
        request_name: PackageName,
        request_spec: String,
        package_id: PackageId,
    ) -> Self {
        Self {
            request_index,
            request_name,
            request_spec,
            package_id,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
