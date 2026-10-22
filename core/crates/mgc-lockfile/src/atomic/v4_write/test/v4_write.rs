//! Pins signed rewrite refusal and public error mapping.
//! Ghim hành vi từ chối rewrite lock có chữ ký và ánh xạ lỗi công khai.

use super::super::{V4WriteError, ensure_v4_unsigned_for_rewrite};

#[test]
fn signed_v4_rewrite_keeps_the_stable_mutation_error() {
    let mut document = crate::canonical::LockfileV4::new("test");
    document.metadata.signature = Some(crate::v4::SignatureBlock {
        algorithm: "ed25519".to_string(),
        key_id: "test-key".to_string(),
        public_key: "test-public-key".to_string(),
        digest: "blake3-test".to_string(),
        signed_at: "2026-10-18T00:00:00Z".to_string(),
        signature: "ed25519-test".to_string(),
    });
    assert!(matches!(
        ensure_v4_unsigned_for_rewrite(&document),
        Err(crate::LockfileError::SignedLockMutation(message))
            if message == "signed v4 mgc.lock cannot be rewritten without explicit re-signing; verify, remove the signature explicitly, then rewrite"
    ));
}

#[test]
fn v4_io_errors_keep_the_source_and_map_to_write_failed() {
    let error = V4WriteError::SyncTemp {
        source: std::io::Error::other("device error"),
    };
    assert_eq!(
        std::error::Error::source(&error)
            .expect("test operation must succeed")
            .to_string(),
        "device error"
    );
    let mapped: crate::LockfileError = error.into();
    assert!(matches!(
        mapped,
        crate::LockfileError::WriteFailed(message)
            if message == "cannot sync lock staging file: device error"
    ));
}
