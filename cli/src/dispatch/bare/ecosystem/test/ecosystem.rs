//! Shared core-resolver contract tests.
//! Test contract cho resolver core dùng chung.
use crate::dispatch::bare::ecosystem::require_ecosystem;

#[test]
fn returns_the_detected_core_without_rewriting() {
    assert_eq!(require_ecosystem("install", Some("web")).unwrap(), "web");
    assert_eq!(
        require_ecosystem("list", Some("future-core")).unwrap(),
        "future-core"
    );
}

#[test]
fn missing_core_preserves_the_fail_closed_diagnostic() {
    let error = require_ecosystem("install", None).expect_err("missing core must fail closed");

    assert_eq!(
        error.to_string(),
        "`mgc install` could not detect a MagiCore core in this project. Use `mgc --core <core> install` or the explicit command such as `mgc install-web`."
    );
}
