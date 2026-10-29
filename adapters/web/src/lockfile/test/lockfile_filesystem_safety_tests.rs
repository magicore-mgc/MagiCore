// lockfile_filesystem_safety_tests — filesystem safety.
// lockfile_filesystem_safety_tests — kiểm thử filesystem safety.
use super::*;

// Symlink creation is a Unix primitive here: `std::os::unix::fs::symlink` does
// not exist on Windows, so an ungated test would break the Windows build rather
// than assert anything. The sibling reader test below is gated the same way.
// (Tạo symlink là primitive Unix: `std::os::unix::fs::symlink` không tồn tại trên
// Windows, nên test không gate sẽ làm build Windows hỏng thay vì assert. Test
// reader bên dưới được gate giống hệt.)
#[cfg(unix)]
#[test]
fn web_lock_writer_refuses_symlink_without_touching_target() {
    use std::os::unix::fs::symlink;

    let dir = tempdir_real().unwrap();
    let target = dir.path().join("external.lock");
    let link = dir.path().join("mgc.lock");
    let original = b"untrusted target";
    std::fs::write(&target, original).unwrap();
    symlink(&target, &link).unwrap();
    let graph = ResolvedGraph::empty();

    assert!(
        write_web_lockfile_with_state(dir.path(), &graph, "https://registry.npmjs.org").is_err()
    );
    assert_eq!(std::fs::read(&target).unwrap(), original);
    assert!(
        std::fs::symlink_metadata(link)
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn web_lock_reader_rejects_oversized_lockfile() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        vec![b'x'; 10 * 1024 * 1024 + 1],
    )
    .unwrap();

    let error = read_web_lockfile_checked(dir.path()).unwrap_err();

    assert!(error.to_string().contains("too large"));
}

#[cfg(unix)]
#[test]
fn web_lock_reader_refuses_symlinked_lockfile() {
    use std::os::unix::fs::symlink;

    let dir = tempdir_real().unwrap();
    let target = dir.path().join("external.lock");
    std::fs::write(&target, "version = \"3\"\n").unwrap();
    symlink(&target, dir.path().join("mgc.lock")).unwrap();

    let error = read_web_lockfile_checked(dir.path()).unwrap_err();

    assert!(error.to_string().contains("symlink"));
    assert_eq!(
        std::fs::read_to_string(target).unwrap(),
        "version = \"3\"\n"
    );
}
