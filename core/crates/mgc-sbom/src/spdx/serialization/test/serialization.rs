#![allow(clippy::unwrap_used)]
//! SPDX serialization cases — Trường hợp serialization SPDX

use super::super::serialize;
use crate::Bom;

#[test]
fn serializes_a_complete_empty_spdx_document() {
    let json = serialize(&Bom::default(), "demo").unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(value["spdxVersion"], "SPDX-2.3");
    assert_eq!(value["SPDXID"], "SPDXRef-DOCUMENT");
    assert_eq!(value["name"], "demo");
    assert_eq!(value["packages"], serde_json::json!([]));
    assert_eq!(value["relationships"], serde_json::json!([]));
}
