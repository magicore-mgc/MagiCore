//! Native remove operation helpers. / Helper thao tác remove native.

use crate::native_ownership_matrix_e2e::support::MatrixSandbox;
use tempfile::TempDir;

/// Native remove cell: add, then remove — the manifest must drop the
/// pin (no stale entry left behind) with zero spawn.
pub(crate) fn native_remove_cell(
    setup: fn(&std::path::Path),
    add_cmd: &[&str],
    remove_cmd: &[&str],
    manifest_file: &str,
    pin: &str,
) {
    native_remove_cell_with_env(setup, add_cmd, remove_cmd, manifest_file, pin, &[]);
}

pub(crate) fn native_remove_cell_with_env(
    setup: fn(&std::path::Path),
    add_cmd: &[&str],
    remove_cmd: &[&str],
    manifest_file: &str,
    pin: &str,
    extra_env: &[(&str, &str)],
) {
    let project = TempDir::new().unwrap();
    setup(project.path());
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(add_cmd, project.path(), extra_env);
    assert_eq!(
        code,
        Some(0),
        "setup {} must succeed:\n{out}",
        add_cmd.join(" ")
    );
    let (code, out) = sandbox.run_with_env(remove_cmd, project.path(), extra_env);
    assert_eq!(
        code,
        Some(0),
        "{} must succeed:\n{out}",
        remove_cmd.join(" ")
    );
    let body = std::fs::read_to_string(project.path().join(manifest_file)).unwrap();
    assert!(!body.contains(pin), "manifest must drop {pin}:\n{body}");
    sandbox.assert_no_spawn(&remove_cmd.join(" "));
}

/// Native remove on a fixture that already carries the dep.
pub(crate) fn native_remove_only(
    setup: fn(&std::path::Path),
    remove_cmd: &[&str],
    manifest_file: &str,
    pin: &str,
) {
    native_remove_only_with_env(setup, remove_cmd, manifest_file, pin, &[]);
}

pub(crate) fn native_remove_only_with_env(
    setup: fn(&std::path::Path),
    remove_cmd: &[&str],
    manifest_file: &str,
    pin: &str,
    extra_env: &[(&str, &str)],
) {
    let project = TempDir::new().unwrap();
    setup(project.path());
    let sandbox = MatrixSandbox::new();
    let body_before = std::fs::read_to_string(project.path().join(manifest_file)).unwrap();
    assert!(
        body_before.contains(pin),
        "fixture must carry {pin}:
{body_before}"
    );
    let (code, out) = sandbox.run_with_env(remove_cmd, project.path(), extra_env);
    assert_eq!(
        code,
        Some(0),
        "{} must succeed:\n{out}",
        remove_cmd.join(" ")
    );
    let body = std::fs::read_to_string(project.path().join(manifest_file)).unwrap();
    assert!(!body.contains(pin), "manifest must drop {pin}:\n{body}");
    sandbox.assert_no_spawn(&remove_cmd.join(" "));
}
