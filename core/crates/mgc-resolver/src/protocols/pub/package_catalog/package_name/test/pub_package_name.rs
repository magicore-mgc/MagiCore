//! Package-name path-segment validation tests.
//! Test kiểm tra tên package không thể thoát khỏi segment URL.

use super::super::validate_pub_package_name;

#[test]
fn pub_package_names_cannot_escape_registry_path_segments() {
    for name in ["../escape", "@scope/pkg", "UpperCase", "foo/bar"] {
        assert!(validate_pub_package_name(name).is_err(), "accepted {name}");
    }
    assert!(validate_pub_package_name("valid_pkg2").is_ok());
}
