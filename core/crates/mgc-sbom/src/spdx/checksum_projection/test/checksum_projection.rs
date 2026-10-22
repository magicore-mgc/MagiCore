#![allow(clippy::unwrap_used)]
//! Checksum projection cases — Trường hợp chiếu checksum

use super::super::super::*;
use crate::cyclonedx::Hash;

#[test]
fn normalizes_sha256_and_lowercases_hex() {
    let checksum = project_checksum(&Hash {
        alg: "sha-256".into(),
        content: "AB".repeat(32),
    })
    .unwrap();

    assert_eq!(checksum.algorithm, "SHA256");
    assert_eq!(checksum.checksum_value, "ab".repeat(32));
}

#[test]
fn omits_unknown_algorithms_and_malformed_values() {
    assert!(
        project_checksum(&Hash {
            alg: "unknown".into(),
            content: "ab".repeat(32),
        })
        .is_none()
    );
    assert!(
        project_checksum(&Hash {
            alg: "SHA256".into(),
            content: "z".repeat(64),
        })
        .is_none()
    );
    assert!(
        project_checksum(&Hash {
            alg: "SHA256".into(),
            content: "ab".repeat(31),
        })
        .is_none()
    );
}
