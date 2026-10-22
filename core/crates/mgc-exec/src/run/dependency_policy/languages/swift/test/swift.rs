// Swift package resolution and offline flag tests.
// Test resolve package và flag offline Swift.

use super::*;

#[test]
fn swift_build_requires_both_update_and_resolution_guards() {
    assert!(denial_reason(&[], "build").is_some());
    assert!(denial_reason(&["--skip-update".into()], "build").is_some());
    assert!(denial_reason(&["--skip-update".into(), "--disable-automatic-resolution".into()], "build").is_none());
    assert!(denial_reason(&["package".into(), "resolve".into()], "package").is_some());
}
