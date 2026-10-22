//! Focused request validation regression tests.
//! Test hồi quy tập trung cho request validation.

use super::*;

#[test]
fn accepts_registry_names_versions_ranges_and_tags() {
    for (request, expected) in [
        ("pkg", ("pkg", "latest")),
        ("@scope/pkg", ("@scope/pkg", "latest")),
        ("pkg@1.2.3", ("pkg", "1.2.3")),
        ("@scope/pkg@^1.2.3", ("@scope/pkg", "^1.2.3")),
        ("pkg@>=1.2.0 <2", ("pkg", ">=1.2.0 <2")),
        ("pkg@beta", ("pkg", "beta")),
    ] {
        assert_eq!(
            parse_request(request).expect("valid registry request"),
            expected
        );
    }
}

#[test]
fn rejects_traversal_urls_controls_and_malformed_names() {
    for request in [
        "",
        "../pkg",
        "/pkg",
        "@scope",
        "@scope/../pkg",
        "@scope/pkg/extra",
        "pkg@",
        "pkg@   ",
        "pkg@../../../../../../escaped",
        "pkg@C:\\escaped",
        "pkg@https://example.invalid/a.tgz",
        "pkg@%2f..",
        "pkg@bad\nvalue",
        "pkg@bad\0value",
        "pkg@@tag",
        "_pkg",
        ".pkg",
    ] {
        assert!(
            parse_request(request).is_err(),
            "unsafe request must be rejected"
        );
    }
    assert!(parse_request(&"a".repeat(MAX_NPM_PACKAGE_NAME_BYTES + 1)).is_err());
    assert!(
        parse_request(&format!(
            "pkg@{}",
            "a".repeat(MAX_REGISTRY_PACKAGE_REQUEST_BYTES)
        ))
        .is_err()
    );
}
