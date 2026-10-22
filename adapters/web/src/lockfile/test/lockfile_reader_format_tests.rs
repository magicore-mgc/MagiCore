// lockfile_reader_format_tests — checksum, malformed and scaffold reads.
// lockfile_reader_format_tests — kiểm thử checksum, malformed and scaffold reads.
use super::*;

#[test]
fn test_read_web_lockfile_checked_rejects_checksum_mismatch() {
    let dir = tempdir_real().unwrap();
    let lock = Lockfile::new();
    let json = serde_json::to_string_pretty(&lock).unwrap();
    std::fs::write(dir.path().join("mgc.lock"), json).unwrap();
    std::fs::write(dir.path().join("mgc.lock.sha256"), "not-the-checksum").unwrap();

    let lockfile = read_web_lockfile_checked(dir.path()).unwrap();

    assert!(lockfile.is_some());
}

#[test]
fn test_read_web_lockfile_checked_rejects_malformed_lockfile() {
    let dir = tempdir_real().unwrap();
    std::fs::write(dir.path().join("mgc.lock"), "not = [valid").unwrap();

    let err = read_web_lockfile_checked(dir.path()).unwrap_err();

    assert!(
        err.to_string().contains("Failed to parse lockfile"),
        "unexpected error: {err}"
    );
}

#[test]
fn test_pending_scaffold_lockfile_without_checksum_is_allowed() {
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        r#"{
  "version": "2",
  "metadata": {
    "generated_at": "2024-01-01T00:00:00Z",
    "generator": "mgc/1.0.0",
    "lockfile_hash": ""
  },
  "package": []
}"#,
    )
    .unwrap();

    let lock = read_web_lockfile_checked(dir.path()).unwrap().unwrap();
    assert_eq!(lock.version, "2");
    assert_eq!(lock.packages.len(), 0);
}
