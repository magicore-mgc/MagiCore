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
fn cargo_project_creation_is_allowed_from_a_local_template() {
    // `cargo new`/`init` render the local template only: no registry, no
    // lockfile write, so framework initializers may run them offline.
    // (`cargo new`/`init` chỉ render template cục bộ: không registry, không ghi
    // lockfile, nên framework initializer chạy được khi offline.)
    assert!(denial_reason(&[], "new").is_none());
    assert!(denial_reason(&[], "init").is_none());
}

#[test]
fn unknown_cargo_subcommands_fail_closed() {
    assert!(denial_reason(&[], "unrecognized-command").is_some());
    assert!(denial_reason(&[], "").is_none());
}
