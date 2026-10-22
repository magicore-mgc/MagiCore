//! Tests for bounded Windows executable hashing.
//! Test băm executable Windows theo luồng có giới hạn.

#[cfg(windows)]
#[test]
fn windows_executable_hash_matches_sha256_across_buffer_boundaries() {
    use sha2::{Digest, Sha256};
    let root = tempfile::tempdir().expect("create hash fixture");
    let path = root.path().join("image.exe");
    for size in [0, 1, 65_536, 131_089] {
        let bytes: Vec<u8> = (0..size).map(|index| (index % 251) as u8).collect();
        std::fs::write(&path, &bytes).expect("write hash fixture");
        let expected: [u8; 32] = Sha256::digest(&bytes).into();
        assert_eq!(super::windows_sha256_file(&path).unwrap(), expected);
    }
    std::fs::remove_file(&path).expect("remove hash fixture");
    assert!(super::windows_sha256_file(&path).is_err());
}
