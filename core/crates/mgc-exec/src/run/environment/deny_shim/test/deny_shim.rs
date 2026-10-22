//! Verify shim bytes, replacement protection, and Windows argument safety.
//! Kiểm tra byte shim, chống ghi đè và an toàn tham số Windows.

use super::super::super::write_blocker;

#[cfg(unix)]
#[test]
fn unix_npm_probe_shim_has_exact_read_only_content() {
    use std::os::unix::fs::PermissionsExt;

    const EXPECTED: &str = concat!(
        "#!/bin/sh\n",
        "if [ \"$#\" -eq 1 ] && [ \"$1\" = \"--version\" ]; then\n",
        "  printf '%s\\n' '0.0.0'\n",
        "  exit 0\n",
        "fi\n",
        "printf '%s\\n' \"MagiCore blocked forbidden package manager: npm\" >&2\n",
        "exit 126\n",
    );
    let root = tempfile::tempdir().expect("create Unix shim test directory");

    write_blocker(root.path(), "npm").expect("write Unix npm shim");

    let path = root.path().join("npm");
    let content = std::fs::read_to_string(&path).expect("read Unix npm shim");
    let mode = std::fs::metadata(path)
        .expect("read Unix npm shim metadata")
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(content, EXPECTED);
    assert_eq!(mode, 0o500);
}

#[cfg(unix)]
#[test]
fn unix_npm_probe_shim_refuses_replacement() {
    let root = tempfile::tempdir().expect("create Unix shim test directory");
    let path = root.path().join("npm");
    std::fs::write(&path, "preserve existing file").expect("write preexisting shim path");

    let error = write_blocker(root.path(), "npm").expect_err("existing shim must not be replaced");

    assert_eq!(error.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(
        std::fs::read_to_string(path).expect("read existing file"),
        "preserve existing file"
    );
}

#[cfg(windows)]
#[test]
fn windows_npm_shim_never_interpolates_caller_arguments() {
    let root = tempfile::tempdir().expect("create Windows shim test directory");

    write_blocker(root.path(), "npm").expect("write Windows npm blocker");

    let content =
        std::fs::read_to_string(root.path().join("npm.cmd")).expect("read Windows npm blocker");
    assert!(content.contains("MagiCore blocked forbidden package manager: npm"));
    assert!(
        !content.contains("%*"),
        "caller arguments must never be expanded"
    );
    assert!(
        !content.contains("%1"),
        "individual caller arguments must not be expanded"
    );
}
