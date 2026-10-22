//! Converts package identities into cache paths.
//! Chuyển định danh package thành đường dẫn cache.

use mgc_types::{PackageId, Version};
use std::path::{Path, PathBuf};

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

fn is_legacy_portable_name(name: &str) -> bool {
    if name.is_empty()
        || !name.is_ascii()
        || name.starts_with('.')
        || name.ends_with('.')
        || name.ends_with(' ')
        || name.contains('~')
        || is_windows_reserved_device_name(name)
    {
        return false;
    }

    name.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'_' | b'-' | b'.' | b'@' | b'!')
    })
}

fn is_windows_reserved_device_name(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name);
    if matches!(stem, "con" | "prn" | "aux" | "nul") {
        return true;
    }

    let bytes = stem.as_bytes();
    bytes.len() == 4
        && (stem.starts_with("com") || stem.starts_with("lpt"))
        && matches!(bytes[3], b'1'..=b'9')
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

fn is_legacy_portable_version(version: &Version) -> bool {
    let rendered = version.to_string();
    if rendered.is_empty()
        || !rendered.is_ascii()
        || rendered.ends_with('.')
        || !rendered.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'-' | b'+' | b'_')
        })
    {
        return false;
    }

    matches!(Version::parse(&rendered), Ok(parsed) if &parsed == version)
}

fn hash_version_identity(version: &Version) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&version.major.to_be_bytes());
    hasher.update(&version.minor.to_be_bytes());
    hasher.update(&version.patch.to_be_bytes());
    hash_optional_string(&mut hasher, version.pre.as_deref());
    hash_optional_string(&mut hasher, version.build.as_deref());
    hash_optional_number(&mut hasher, version.dev);
    hash_optional_number(&mut hasher, version.post);
    hasher.finalize().to_hex().to_string()
}

fn hash_optional_string(hasher: &mut blake3::Hasher, value: Option<&str>) {
    match value {
        Some(value) => {
            hasher.update(&[1]);
            hasher.update(&(value.len() as u128).to_be_bytes());
            hasher.update(value.as_bytes());
        }
        None => {
            hasher.update(&[0]);
        }
    }
}

fn hash_optional_number(hasher: &mut blake3::Hasher, value: Option<u64>) {
    match value {
        Some(value) => {
            hasher.update(&[1]);
            hasher.update(&value.to_be_bytes());
        }
        None => {
            hasher.update(&[0]);
        }
    }
}

pub(in crate::cache) fn metadata_path(root: &Path, name: &str) -> PathBuf {
    cache_name_directory(root, name).join("metadata.json")
}
