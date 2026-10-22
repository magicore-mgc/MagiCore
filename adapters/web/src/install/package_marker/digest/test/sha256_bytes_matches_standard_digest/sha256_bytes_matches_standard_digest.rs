// Pin a known SHA-256 result — Ghim kết quả SHA-256 đã biết.
#[test]
fn sha256_bytes_matches_standard_digest() {
    assert_eq!(
        compute_sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
