//! `process_table.rs` — Covers exact process-table rows, shim path spacing and fake suffix rejection..
//! Bao phủ row process-table chính xác, khoảng trắng path shim và từ chối suffix giả.
use super::super::*;

#[cfg(unix)]
#[test]
fn process_table_accepts_linux_npm_comm_with_its_interpreter_command_line() {
    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor-shim"), &[])
        .expect("create authenticated executor shims");
    let command_line = format!("/bin/sh {} --version", shadow.path().join("npm").display());
    let output = process_table_with_child("npm", &command_line);

    let result = inspect_forbidden_process_table(4242, output, &[], shadow.path());

    assert!(
        matches!(result, Ok(None)),
        "an exact executor shim version probe should be allowed: {result:?}"
    );
}

#[cfg(unix)]
#[test]
fn process_table_preserves_repeated_spaces_in_the_executor_shim_path() {
    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor  shim"), &[])
        .expect("create authenticated executor shims");
    let command_line = format!("/bin/sh {} --version", shadow.path().join("npm").display());
    let output = process_table_with_child("sh", &command_line);

    let result = inspect_forbidden_process_table(4242, output, &[], shadow.path());

    assert!(
        matches!(result, Ok(None)),
        "process inspection must preserve the exact shim path: {result:?}"
    );
}

#[cfg(unix)]
#[test]
fn process_table_rejects_a_real_npm_command_with_a_fake_shim_suffix() {
    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor-shim"), &[])
        .expect("create authenticated executor shims");
    let command_line = format!(
        "/bin/sh /real/path/npm --version {} --version",
        shadow.path().join("npm").display()
    );
    let output = process_table_with_child("npm", &command_line);

    let result = inspect_forbidden_process_table(4242, output, &[], shadow.path());

    assert!(
        matches!(result, Ok(Some(_))),
        "a real npm invocation must remain forbidden: {result:?}"
    );
}
