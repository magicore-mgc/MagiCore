//! Known-output tests for registry digest helpers.
//! Test kết quả chuẩn cho helper digest registry.

#[test]
fn sha256_hex_matches_known_digest() {
    assert_eq!(
        super::super::digest::sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn blake3_store_reference_keeps_two_character_shard() {
    let digest = "abcdef0123456789";
    assert_eq!(
        super::super::digest::store_ref_for_blake3(digest),
        "files/blake3/ab/abcdef0123456789"
    );
}
