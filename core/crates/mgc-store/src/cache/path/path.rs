//! Converts package identities into cache paths.
//! Chuyển định danh package thành đường dẫn cache.

use mgc_types::PackageId;
use std::path::{Path, PathBuf};

use super::naming::{hash_version_identity, is_legacy_portable_name, is_legacy_portable_version};

const HASHED_NAME_NAMESPACE: &str = ".mgc-cache-name-v1";
const HASHED_VERSION_NAMESPACE: &str = ".mgc-cache-version-v1";

/// Select a portable cache directory without aliasing distinct identities.
/// Chọn thư mục cache portable mà không để hai identity khác nhau bị trùng.
///
/// Raw metadata names can contain path-special values, and package names can
/// flatten to the same legacy path. Preserve paths only for a conservative
/// portable subset; place all other names under a versioned hash namespace.
/// Metadata thô có thể chứa giá trị đặc biệt của path; tên package cũng có
/// thể cùng rơi vào một path cũ. Chỉ giữ path cho tập tên portable an toàn,
/// các tên còn lại nằm trong namespace hash có version.
fn cache_name_directory(root: &Path, name: &str) -> PathBuf {
    match name {
        "." => root.join("~d"),
        ".." => root.join("~dd"),
        _ if is_legacy_portable_name(name) => root.join(name),
        _ => hashed_name_directory(root, name),
    }
}

fn hashed_name_directory(root: &Path, name: &str) -> PathBuf {
    let digest = blake3::hash(name.as_bytes()).to_hex().to_string();
    root.join(HASHED_NAME_NAMESPACE).join(digest)
}

pub(in crate::cache) fn tarball_path(root: &Path, id: &PackageId) -> PathBuf {
    let package_directory = cache_name_directory(root, id.name_str());
    if is_legacy_portable_version(id.version()) {
        package_directory.join(format!("{}.tgz", id.version()))
    } else {
        let digest = hash_version_identity(id.version());
        package_directory
            .join(HASHED_VERSION_NAMESPACE)
            .join(format!("{digest}.tgz"))
    }
}

pub(in crate::cache) fn metadata_path(root: &Path, name: &str) -> PathBuf {
    cache_name_directory(root, name).join("metadata.json")
}
