//! Tests for the no-hook execution path without process spawning.
//! Test nhánh không có hook và không spawn process.

use super::super::run_hooks_with_config;
use crate::hooks::HooksConfig;

#[test]
fn absent_event_returns_without_spawning_a_process() {
    let root = tempfile::tempdir().unwrap();
    assert!(
        run_hooks_with_config(
            root.path(),
            "s525-no-configured-event",
            &HooksConfig::default()
        )
        .is_ok()
    );
}
