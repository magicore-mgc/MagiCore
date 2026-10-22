#![cfg(test)]

//! Verify explicit Maven store root overrides the managed default.
//! Xác nhận override Maven store root được ưu tiên hơn mặc định.

use crate::native::rn_layers::install_maven::install_maven::maven_store_root_path;

#[test]
fn explicit_maven_store_root_is_preserved() {
    let expected = std::env::temp_dir().join("mgc-maven-test");
    let override_value = expected.to_string_lossy().into_owned();
    let path = maven_store_root_path(Some(override_value));
    assert_eq!(path, expected);
}

#[test]
fn whitespace_override_uses_managed_store_suffix() {
    let path = maven_store_root_path(Some("   ".into()));
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("maven")
    );
    assert_eq!(
        path.parent()
            .and_then(std::path::Path::file_name)
            .and_then(|name| name.to_str()),
        Some("store")
    );
}
