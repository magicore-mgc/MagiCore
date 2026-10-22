//! Sidecar path cases — kiểm tra quy tắc thay extension.

#![cfg(test)]

use super::sidecar_path::metadata_path;
use std::path::Path;

#[test]
fn replaces_only_the_final_extension_and_preserves_parent() {
    assert_eq!(
        metadata_path(Path::new("/cache/model.tar.bin")),
        Path::new("/cache/model.tar.meta.json")
    );
}

#[test]
fn appends_sidecar_extension_for_extensionless_path() {
    assert_eq!(
        metadata_path(Path::new("/cache/model")),
        Path::new("/cache/model.meta.json")
    );
}
