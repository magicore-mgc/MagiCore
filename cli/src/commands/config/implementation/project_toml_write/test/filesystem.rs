use super::super::*;
use std::fs;

#[cfg(unix)]
#[test]
fn set_toml_rejects_symlink_without_modifying_target() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("outside.toml");
    let path = dir.path().join("mgc.toml");
    fs::write(&target, "name = \"outside\"\necosystem = \"web\"\n").unwrap();
    let before = fs::read(&target).unwrap();
    symlink(&target, &path).unwrap();

    let err = set_toml_at(&path, "name", "overwritten").unwrap_err();

    assert!(err.to_string().contains("must not be a symlink"));
    assert_eq!(fs::read(&target).unwrap(), before);
}

#[cfg(unix)]
#[test]
fn delete_toml_rejects_symlink_without_modifying_target() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("outside.toml");
    let path = dir.path().join("mgc.toml");
    fs::write(&target, "name = \"outside\"\necosystem = \"web\"\n").unwrap();
    let before = fs::read(&target).unwrap();
    symlink(&target, &path).unwrap();

    let err = delete_toml_at(&path, "name").unwrap_err();

    assert!(err.to_string().contains("must not be a symlink"));
    assert_eq!(fs::read(&target).unwrap(), before);
}

#[cfg(unix)]
#[test]
fn config_toml_mutations_reject_fifo_without_blocking() {
    use std::os::unix::fs::FileTypeExt;

    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.toml");
    let status = std::process::Command::new("mkfifo")
        .arg(&path)
        .status()
        .expect("POSIX test runner must provide mkfifo");
    assert!(status.success(), "mkfifo exited with {status}");

    let set_result = std::thread::spawn({
        let path = path.clone();
        move || set_toml_at(&path, "name", "blocked")
    })
    .join()
    .unwrap();
    let delete_result = std::thread::spawn({
        let path = path.clone();
        move || delete_toml_at(&path, "name")
    })
    .join()
    .unwrap();

    assert!(set_result.is_err());
    assert!(delete_result.is_err());
    assert!(
        std::fs::symlink_metadata(path)
            .unwrap()
            .file_type()
            .is_fifo()
    );
}
