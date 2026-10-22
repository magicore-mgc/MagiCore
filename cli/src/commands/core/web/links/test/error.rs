//! Web package-link error contract tests.
//! Test contract lỗi link package Web.

use super::super::error::WebLinkError;
use std::error::Error as StdError;

#[test]
fn local_typed_errors_preserve_established_diagnostics() {
    assert_eq!(
        WebLinkError::unsupported_core("link").to_string(),
        "link only supported for web core"
    );
    assert_eq!(
        WebLinkError::missing_package("link").to_string(),
        "Usage: mgc link <package>"
    );
    assert_eq!(
        WebLinkError::link_exists("sample").to_string(),
        "sample already exists in node_modules"
    );
    assert_eq!(
        WebLinkError::unlink_not_linked("sample").to_string(),
        "sample is not linked in node_modules"
    );
    assert_eq!(
        WebLinkError::not_package_link("sample").to_string(),
        "refusing to remove non-link package entry 'sample' from node_modules"
    );
    assert_eq!(
        WebLinkError::unsafe_link_parent("@scope/sample").to_string(),
        "refusing to access package link '@scope/sample' through a symlink or non-directory scope"
    );
    let project = tempfile::tempdir().expect("tempdir");
    let missing_path = project.path().join("missing");
    assert_eq!(
        WebLinkError::local_path_not_found(&missing_path).to_string(),
        format!("local package path not found: {}", missing_path.display())
    );
    assert_eq!(
        WebLinkError::package_not_found("sample").to_string(),
        concat!(
            "package 'sample' not found in local node_modules or MagiCore global ",
            "package roots.\nUse 'mgc install' to install it first, provide a ",
            "local path, or configure MAGICORE_GLOBAL_PACKAGE_ROOT."
        )
    );
    assert!(StdError::source(&WebLinkError::missing_package("link")).is_none());
}

#[test]
fn filesystem_error_keeps_its_source() {
    let error = WebLinkError::filesystem(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "permission denied",
    ));
    assert_eq!(error.to_string(), "permission denied");
    assert_eq!(
        StdError::source(&error)
            .expect("filesystem cause")
            .to_string(),
        "permission denied"
    );
}
