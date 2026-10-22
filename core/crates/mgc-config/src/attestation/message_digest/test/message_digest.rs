#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn canonical_message_is_domain_separated_and_digest_is_stable_hex() {
    assert_eq!(
        canonical_message("web", "abc"),
        "mgc-core-attestation/v1\nweb\nabc"
    );
    let first = blake3_hex(b"config bytes");
    assert_eq!(first, blake3_hex(b"config bytes"));
    assert_eq!(first.len(), 64);
    assert!(
        first
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    );
}
