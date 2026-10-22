// Rust compiler probe restriction tests.
// Test giới hạn probe compiler Rust.

use super::*;

#[test]
fn rustc_allows_only_exact_host_query() {
    assert!(
        crate::allowlist::check_tool_with_scope(
            "rustc",
            crate::allowlist::ExecutionScope::BuildRunner,
            None,
        )
        .is_ok()
    );
    assert!(denial_reason(&["-vV".into()]).is_none());
    assert!(denial_reason(&["-vv".into()]).is_some());
    assert!(denial_reason(&["--version".into()]).is_some());
    assert!(denial_reason(&["-vv".into(), "--crate-name".into(), "injected".into()]).is_some());
}
