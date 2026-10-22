#![allow(clippy::unwrap_used)]
//! Hash serialization tests — Test tuần tự hóa hash

use super::super::super::*;

#[test]
fn serializes_algorithm_and_hex_content() {
    let value = serde_json::to_value(Hash {
        alg: "SHA256".into(),
        content: "ab".repeat(32),
    })
    .unwrap();

    assert_eq!(value["alg"], "SHA256");
    assert_eq!(value["content"].as_str().unwrap().len(), 64);
}
