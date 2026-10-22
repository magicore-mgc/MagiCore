#![allow(clippy::unwrap_used)]
//! SPDX checksum serde tests — Test serde checksum SPDX

use super::super::super::*;

#[test]
fn serializes_the_algorithm_and_checksum_value() {
    let value = serde_json::to_value(SpdxChecksum {
        algorithm: "SHA256",
        checksum_value: "ab".repeat(32),
    })
    .unwrap();

    assert_eq!(value["algorithm"], "SHA256");
    assert_eq!(value["checksumValue"].as_str().unwrap().len(), 64);
}
