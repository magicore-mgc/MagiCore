//! Verify bounded member comparison, digest and final-link rejection.
//! Kiểm tra đối chiếu member có giới hạn, digest và từ chối link cuối.

use super::super::compare_installed_member;
use sha2::{Digest, Sha256};

#[test]
fn record_verification_streams_large_matching_member_and_returns_sha256() {
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let path = temp.path().join("module.py");
    let expected = vec![b'x'; 256 * 1024 + 19];
    std::fs::write(&path, &expected).expect("test operation must succeed");

    let (size, digest) = compare_installed_member(&path, "pkg/module.py", &expected)
        .expect("test operation must succeed");
    assert_eq!(size, expected.len() as u64);
    assert_eq!(digest.as_slice(), Sha256::digest(&expected).as_slice());
}

#[test]
fn record_verification_rejects_content_or_length_mismatch() {
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let path = temp.path().join("module.py");
    std::fs::write(&path, b"changed payload").expect("test operation must succeed");

    let error = compare_installed_member(&path, "pkg/module.py", b"trusted payload")
        .expect_err("test operation must fail");
    assert!(
        error
            .to_string()
            .contains("differs from authenticated wheel member")
    );
}

#[cfg(unix)]
#[test]
fn record_verification_does_not_follow_a_final_symlink() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().expect("test operation must succeed");
    let outside = temp.path().join("outside.py");
    let link = temp.path().join("module.py");
    std::fs::write(&outside, b"trusted").expect("test operation must succeed");
    symlink(&outside, &link).expect("test operation must succeed");

    let error = compare_installed_member(&link, "pkg/module.py", b"trusted")
        .expect_err("test operation must fail");
    assert!(error.to_string().contains("unsafe file"));
}
