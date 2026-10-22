#![allow(clippy::unwrap_used)]
//! License serialization tests — Test tuần tự hóa license

use super::super::super::*;

#[test]
fn omits_absent_id_and_name_values() {
    let value = serde_json::to_value(License {
        id: None,
        name: None,
    })
    .unwrap();
    assert!(value.get("id").is_none());
    assert!(value.get("name").is_none());
}
