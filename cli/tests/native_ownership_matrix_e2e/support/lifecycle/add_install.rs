//! Native add/install and blocked-mutation helpers. / Helper add/install native và chặn mutation.

use crate::native_ownership_matrix_e2e::support::MatrixSandbox;
use tempfile::TempDir;

/// Native add cell: succeeds + manifest pinned + zero spawn.
pub(crate) fn native_add_cell_with_env(
    setup: fn(&std::path::Path),
    add_cmd: &[&str],
    manifest_file: &str,
    pin: &str,
    lock_pin: &str,
    extra_env: &[(&str, &str)],
) {
    let project = TempDir::new().unwrap();
    setup(project.path());
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run_with_env(add_cmd, project.path(), extra_env);
    assert_eq!(code, Some(0), "{} must succeed:\n{out}", add_cmd.join(" "));
    let body = std::fs::read_to_string(project.path().join(manifest_file)).unwrap();
    assert!(body.contains(pin), "manifest must pin {pin}:\n{body}");
    sandbox.assert_no_spawn(&add_cmd.join(" "));
    let lock = std::fs::read_to_string(project.path().join("mgc.lock")).unwrap_or_default();
    assert!(
        lock.contains(lock_pin),
        "mgc.lock must record {lock_pin}:\n{lock}"
    );
}

/// Native install cell (after add): succeeds + zero spawn.
pub(crate) fn native_install_cell_with_env(
    setup: fn(&std::path::Path),
    add_cmd: &[&str],
    install_cmd: &[&str],
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
    let (code, out) = sandbox.run_with_env(install_cmd, project.path(), extra_env);
    assert_eq!(
        code,
        Some(0),
        "{} must succeed:\n{out}",
        install_cmd.join(" ")
    );
    sandbox.assert_no_spawn(&install_cmd.join(" "));
}

/// A default-blocked mutation must preserve its manifest byte-for-byte.
/// (Mutation bị chặn mặc định phải giữ manifest nguyên từng byte.)
pub(crate) fn blocked_mutation_unchanged(
    setup: fn(&std::path::Path),
    cmd: &[&str],
    manifest_file: &str,
) {
    let project = TempDir::new().unwrap();
    setup(project.path());
    let manifest = project.path().join(manifest_file);
    let before = std::fs::read(&manifest).unwrap();
    let sandbox = MatrixSandbox::new();
    let (code, out) = sandbox.run(cmd, project.path());
    assert_ne!(
        code,
        Some(0),
        "{} must refuse without compat:\n{out}",
        cmd.join(" ")
    );
    assert_eq!(
        std::fs::read(&manifest).unwrap(),
        before,
        "blocked command mutated {manifest_file}"
    );
    sandbox.assert_no_spawn(&cmd.join(" "));
}
