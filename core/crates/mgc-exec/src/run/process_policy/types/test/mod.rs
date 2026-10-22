//! Shared ForbiddenProcess data contract test.
//! Test contract dữ liệu ForbiddenProcess dùng chung.

use super::*;

#[test]
fn forbidden_process_retains_match_evidence_without_owning_raw_argv() {
    let found = ForbiddenProcess {
        pid: 42,
        name: "npm".to_owned(),
        image_name: Some("npm.exe".to_owned()),
        match_source: "runtime entrypoint",
        matched_token: Some("npm-cli.js".to_owned()),
        matched_token_index: Some(3),
        matched_operation: "dependency-mutation invocation".to_owned(),
    };
    assert_eq!(found.pid, 42);
    assert_eq!(found.name, "npm");
    assert_eq!(found.match_source, "runtime entrypoint");
    assert_eq!(found.matched_token_index, Some(3));
    assert!(!format!("{found:?}").contains("secret-package-argument"));
}
