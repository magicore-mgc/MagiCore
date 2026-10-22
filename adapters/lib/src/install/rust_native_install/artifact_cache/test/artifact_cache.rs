//! Tests cached archive verification and link rejection.
//! Kiểm tra xác minh archive cache và từ chối link.

use crate::install::rust_native_install::read_cached_rust_artifact;
use sha2::Digest;

#[test]
fn rust_artifact_cache_refuses_corruption_and_symlinks() {
    let cache = tempfile::tempdir().unwrap();
    let artifact = cache.path().join("crate.crate");
    let bytes = b"crate archive";
    let expected = hex::encode(sha2::Sha256::digest(bytes));
    std::fs::write(&artifact, bytes).unwrap();
    assert_eq!(
        read_cached_rust_artifact(&artifact, &expected).unwrap(),
        Some(bytes.to_vec())
    );
    std::fs::write(&artifact, b"tampered crate").unwrap();
    assert!(
        read_cached_rust_artifact(&artifact, &expected)
            .unwrap_err()
            .to_string()
            .contains("integrity mismatch")
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let outside = tempfile::tempdir().unwrap();
        let target = outside.path().join("crate.crate");
        std::fs::write(&target, bytes).unwrap();
        std::fs::remove_file(&artifact).unwrap();
        symlink(target, &artifact).unwrap();
        assert!(
            read_cached_rust_artifact(&artifact, &expected)
                .unwrap_err()
                .to_string()
                .contains("regular file")
        );
    }
}
