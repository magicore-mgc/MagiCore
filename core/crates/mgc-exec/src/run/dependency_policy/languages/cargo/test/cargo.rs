// Cargo dependency resolution boundary tests.
// Test ranh giới resolve dependency Cargo.

use super::*;

#[test]
fn cargo_mutation_and_registry_commands_are_denied() {
    for command in [
        "add",
        "remove",
        "update",
        "fetch",
        "install",
        "publish",
        "tree",
        "info",
        "package",
        "generate-lockfile",
        "upgrade",
    ] {
        assert!(denial_reason(&[], command).is_some(), "cargo {command} must be denied");
    }
}

#[test]
fn cargo_build_requires_both_locked_and_offline() {
    assert!(denial_reason(&[], "build").is_some());
    assert!(denial_reason(&["--offline".into()], "build").is_some());
    assert!(denial_reason(&["--locked".into(), "--offline".into()], "build").is_none());
}

#[test]
fn unknown_cargo_subcommands_fail_closed() {
    assert!(denial_reason(&[], "unrecognized-command").is_some());
    assert!(denial_reason(&[], "").is_none());
}
