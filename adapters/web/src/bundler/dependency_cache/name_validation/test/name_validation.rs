//! Accepted and rejected dependency-name shape tests.
//! Test các dạng tên dependency được nhận và bị từ chối.

use super::dep_name_is_valid;

#[test]
fn accepts_plain_scoped_and_package_subpath_names() {
    for name in [
        "react",
        "lodash/fp",
        "lodash/fp/map",
        "@scope/package",
        "@scope/package/sub/path",
    ] {
        assert!(
            dep_name_is_valid(name),
            "expected valid package name: {name}"
        );
    }
}

#[test]
fn rejects_empty_traversal_encoding_and_host_path_segments() {
    for name in [
        "",
        "@",
        "@scope",
        "@scope%2fescape/package",
        "@scope//package",
        "../outside",
        "@scope/../outside",
        "%2e%2e",
        "C:/outside",
        "@scope/C:/outside",
        "NUL",
        "nul.txt",
        "NUL .txt",
        "CONIN$",
        "COM1",
        "COM¹",
        "LPT9.log",
        "trailing.",
        "trailing ",
        "bad<name",
        "bad?name",
        r"plain\..\outside",
        r"@scope/pkg\..\outside",
        "plain:stream",
        "@scope/pkg:stream",
        "nul\0name",
        "control\u{7}name",
        "line\u{2028}separator",
        "paragraph\u{2029}separator",
    ] {
        assert!(
            !dep_name_is_valid(name),
            "expected invalid package name: {name:?}"
        );
    }
}

#[test]
fn rejects_segments_over_the_existing_length_limit() {
    let name = "a".repeat(215);

    assert!(!dep_name_is_valid(&name));
}
