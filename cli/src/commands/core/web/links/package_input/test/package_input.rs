//! Tests for Web package-link input ordering.
//! Test thứ tự kiểm tra đầu vào link package Web.

use super::super::implementation::require_web_package;

#[test]
fn non_web_core_is_rejected_before_missing_package() {
    let error = require_web_package("app", "link", None).expect_err("reject foreign core");
    assert_eq!(error.to_string(), "link only supported for web core");
}

#[test]
fn missing_package_uses_the_command_specific_usage() {
    for command in ["link", "unlink"] {
        let error = require_web_package("web", command, None).expect_err("require package");
        assert_eq!(error.to_string(), format!("Usage: mgc {command} <package>"));
    }
}

#[test]
fn package_argument_is_returned_unchanged() {
    assert_eq!(
        require_web_package("web", "link", Some("@scope/package")).expect("valid package"),
        "@scope/package"
    );
}
