//! Check lock-digest preflight before untrusted ZIP parsing.
//! Kiểm tra preflight digest lock trước khi parse ZIP không tin cậy.

use crate::protocols::pypi::PypiProtocol;

#[test]
fn wheel_record_parser_facade_preserves_legacy_error_type_and_text() {
    assert_eq!(
        super::super::parse_record_row("only,two"),
        Err("expected exactly three CSV fields")
    );
}

#[test]
fn wheel_record_rejects_malformed_lock_digest_before_parsing_archive() {
    let error = PypiProtocol::verify_runtime_materialization(
        b"not a zip archive",
        "demo",
        "1.0.0",
        "invalid-digest",
        std::path::Path::new("unused-site-path"),
    )
    .expect_err("test operation must fail");

    assert!(
        error
            .to_string()
            .contains("Python runtime lock entry has no valid SHA-256 digest")
    );
}
