// Zig dependency-free execution tests.
// Test chạy Zig không dependency.

use super::*;

#[test]
fn only_version_and_direct_source_run_are_allowed() {
    assert!(denial_reason(&["version".into()]).is_none());
    assert!(denial_reason(&["run".into(), "probe.zig".into()]).is_none());
    assert!(denial_reason(&["fetch".into(), "https://example.invalid/pkg".into()]).is_some());
    assert!(denial_reason(&["build".into()]).is_some());
    assert!(denial_reason(&["run".into(), "probe.zig".into(), "--fetch".into()]).is_some());
}
