//! `shim_security.rs` — Covers shim tampering and invalid Unix path-byte separation..
//! Bao phủ sửa shim trái phép và phân biệt byte path Unix không hợp lệ.
use super::super::*;
use std::fs;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[cfg(unix)]
#[test]
fn process_table_rejects_a_modified_npm_probe_shim() {
    use std::os::unix::fs::PermissionsExt;

    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor-shim"), &[])
        .expect("create authenticated executor shims");
    let mut directory_permissions = fs::metadata(shadow.path())
        .expect("read sealed executor directory permissions")
        .permissions();
    directory_permissions.set_mode(0o700);
    fs::set_permissions(shadow.path(), directory_permissions)
        .expect("temporarily make executor directory writable for tampering test");
    let shim_path = shadow.path().join("npm");
    let mut shim_permissions = fs::metadata(&shim_path)
        .expect("read sealed shim permissions")
        .permissions();
    shim_permissions.set_mode(0o600);
    fs::set_permissions(&shim_path, shim_permissions)
        .expect("temporarily make npm shim writable for tampering test");
    fs::write(&shim_path, "#!/bin/sh\nnpm install\n")
        .expect("replace npm shim with a forbidden command");
    let mut shim_permissions = fs::metadata(&shim_path)
        .expect("read modified shim permissions")
        .permissions();
    shim_permissions.set_mode(0o500);
    fs::set_permissions(&shim_path, shim_permissions).expect("reseal modified npm shim");
    let mut directory_permissions = fs::metadata(shadow.path())
        .expect("read writable executor directory permissions")
        .permissions();
    directory_permissions.set_mode(0o500);
    fs::set_permissions(shadow.path(), directory_permissions)
        .expect("reseal modified executor directory");

    let command_line = format!("/bin/sh {} --version", shim_path.display());
    let output = process_table_with_child("sh", &command_line);
    let result = inspect_forbidden_process_table(4242, output, &[], shadow.path());

    assert!(
        matches!(result, Ok(Some(_))),
        "a modified executor shim must no longer qualify for the read-only probe: {result:?}"
    );
}

#[cfg(unix)]
#[test]
fn process_table_rejects_a_different_invalid_utf8_shim_path() {
    let root = tempfile::tempdir().expect("create process-table test directory");
    let shadow = ShadowPath::create_at(root.path().join("executor-shim"), &[])
        .expect("create authenticated executor shims");
    let shim_path = shadow.path().join("npm");
    let impostor_line = b"/bin/sh /tmp/executor-\xff/npm --version".to_vec();
    assert!(
        !impostor_line
            .windows(shim_path.as_os_str().as_bytes().len())
            .any(|window| window == shim_path.as_os_str().as_bytes()),
        "the synthetic invalid path must differ from the real shim path"
    );
    let output = process_table_with_raw_child(b"sh", &impostor_line);
    let impostor_result = inspect_forbidden_process_table(4242, output, &[], shadow.path());
    assert!(
        matches!(impostor_result, Ok(Some(_))),
        "a different invalid-UTF-8 path must not authenticate the real shim: {impostor_result:?}"
    );
}

#[cfg(unix)]
#[test]
fn npm_probe_path_comparison_keeps_invalid_unix_bytes_distinct() {
    let expected = b"/tmp/executor-\xff/npm";
    let exact = b"/bin/sh /tmp/executor-\xff/npm --version";
    let different = b"/bin/sh /tmp/executor-\xfe/npm --version";

    assert!(npm_probe_command_line_matches_shim_path(
        "sh", exact, expected
    ));
    assert!(!npm_probe_command_line_matches_shim_path(
        "sh", different, expected
    ));
}
