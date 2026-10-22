//! Tests checksum algorithms and fail-closed diagnostics.
//! Kiểm tra thuật toán checksum và diagnostic fail-closed.

use crate::install::download::verify_file_checksum;
use tempfile::TempDir;

fn tmp() -> TempDir {
    TempDir::new().unwrap()
}

#[test]
fn test_verify_file_checksum_algorithms() {
    let tmp = tmp();
    let file = tmp.path().join("m.bin");
    std::fs::write(&file, b"hello magi").unwrap();

    use sha2::Digest;
    let sha = hex::encode(sha2::Sha256::digest(b"hello magi"));
    let blake = mgc_crypto::Blake3Hasher::hash_bytes(b"hello magi").to_hex();

    verify_file_checksum(&file, &format!("sha256:{sha}")).unwrap();
    verify_file_checksum(&file, &format!("blake3:{blake}")).unwrap();
    verify_file_checksum(&file, &blake).unwrap(); // bare hex = blake3

    let err = verify_file_checksum(&file, "sha256:0000").unwrap_err();
    assert!(err.to_string().contains("checksum mismatch"));

    let err2 = verify_file_checksum(&file, "md5:abc").unwrap_err();
    assert!(err2.to_string().contains("unsupported checksum algorithm"));
}
