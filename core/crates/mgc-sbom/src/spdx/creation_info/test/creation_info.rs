#![allow(clippy::unwrap_used)]
//! Creation-info serde tests — Test serde creation-info

use super::super::super::*;

#[test]
fn serializes_created_and_creators_in_camel_case() {
    let value = serde_json::to_value(CreationInfo {
        creators: vec!["Tool: mgc".into()],
        created: "2026-10-17T00:00:00Z".into(),
    })
    .unwrap();

    assert_eq!(value["creators"][0], "Tool: mgc");
    assert_eq!(value["created"], "2026-10-17T00:00:00Z");
}
