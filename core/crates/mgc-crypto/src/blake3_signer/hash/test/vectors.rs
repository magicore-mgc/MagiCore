//! Tests digest vectors and verification behavior.
//! Kiểm tra vector digest và hành vi xác minh.

use crate::blake3_signer::{Blake3Hash, Blake3Hasher};

#[test]
fn known_hello_world_vector_and_verify() {
    let hash = Blake3Hasher::hash_string("hello world");
    assert_eq!(
        hash.to_hex(),
        "d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24"
    );
    assert!(Blake3Hasher::verify(b"hello world", &hash));
    assert!(!Blake3Hasher::verify(b"different", &hash));
    assert_eq!(
        Blake3Hash::from_hex(&hash.to_hex()).expect("test operation must succeed"),
        hash
    );
}
