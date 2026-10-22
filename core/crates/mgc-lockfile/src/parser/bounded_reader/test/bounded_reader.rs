//! Checks exact caps and final-component symlink rejection.
//! Kiểm tra giới hạn byte và từ chối symlink ở path cuối.

use super::super::read_bounded_regular_file;
use tempfile::tempdir;

#[test]
fn reader_accepts_at_cap_and_rejects_one_byte_over() {
    let directory = tempdir().expect("test operation must succeed");
    let path = directory.path().join("input.lock");
    std::fs::write(&path, b"four").expect("test operation must succeed");
    assert_eq!(
        read_bounded_regular_file(&path, 4, "input").expect("test operation must succeed"),
        b"four"
    );
    std::fs::write(&path, b"five!").expect("test operation must succeed");
    assert!(
        read_bounded_regular_file(&path, 4, "input")
            .expect_err("test operation must fail")
            .to_string()
            .contains("too large")
    );
}

#[cfg(unix)]
#[test]
fn reader_rejects_a_final_symlink() {
    let directory = tempdir().expect("test operation must succeed");
    let target = directory.path().join("target");
    let link = directory.path().join("input.lock");
    std::fs::write(&target, b"data").expect("test operation must succeed");
    std::os::unix::fs::symlink(&target, &link).expect("test operation must succeed");
    assert!(
        read_bounded_regular_file(&link, 8, "input")
            .expect_err("test operation must fail")
            .to_string()
            .contains("regular non-symlink")
    );
    assert_eq!(
        std::fs::read(target).expect("test operation must succeed"),
        b"data"
    );
}
