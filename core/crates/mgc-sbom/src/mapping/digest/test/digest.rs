#![allow(clippy::unwrap_used)]
//! Digest validation owner tests — Kiểm thử owner xác thực digest

use super::super::*;

#[test]
fn accepts_only_sixty_four_hexadecimal_bytes() {
    assert!(is_hex_digest(&"a".repeat(64)));
    assert!(!is_hex_digest(&"a".repeat(63)));
    assert!(!is_hex_digest(&format!("{}g", "a".repeat(63))));
}
