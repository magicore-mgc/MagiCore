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
        assert!(
            denial_reason(&[], command).is_some(),
            "cargo {command} must be denied"
        );
    }
}

#[test]
fn cargo_build_requires_locked() {
    assert!(denial_reason(&[], "build").is_some());
    assert!(denial_reason(&["--offline".into()], "build").is_some());
    assert!(denial_reason(&["--locked".into()], "build").is_none());
}

#[test]
fn cargo_offline_stays_an_explicit_choice() {
    // `--offline` was mandatory, so `cargo build` could never fetch a crate the
    // local registry did not hold and `mgc build` had no way to opt in to a
    // network fetch. The locked/offline pair must both still be accepted.
    // (`--offline` từng bắt buộc nên build không bao giờ tải được crate còn
    // thiếu; cặp locked/offline vẫn phải được chấp nhận.)
    assert!(denial_reason(&["--locked".into(), "--offline".into()], "build").is_none());
    for command in ["test", "check", "run", "clippy"] {
        assert!(
            denial_reason(&["--locked".into()], command).is_none(),
            "cargo {command} --locked must be allowed online"
        );
        assert!(
            denial_reason(&["--locked".into(), "--offline".into()], command).is_none(),
            "cargo {command} --locked --offline must stay allowed"
        );
    }
}

#[test]
fn cargo_metadata_is_a_read_only_query() {
    // `cargo metadata` reads the graph; it neither builds nor fetches deps.
    // (`cargo metadata` chỉ đọc graph, không build hay tải dependency.)
    assert!(denial_reason(&[], "metadata").is_none());
    assert!(denial_reason(&["--locked".into(), "--offline".into()], "metadata").is_none());
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
