//! Preserve source defaults and required registry identity fields.
//! Giữ default source và trường identity registry bắt buộc.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn source_ref_defaults_optional_selection_and_trust_fields() {
    let source: SourceRef = serde_json::from_str(
        r#"{"id":"npm-public","url":"https://registry.example","ecosystem":"web"}"#,
    )
    .expect("test operation must succeed");
    assert_eq!(source.priority, 0);
    assert!(source.claims.is_empty());
    assert!(!source.trusted);
    assert!(source.allow_hosts.is_empty());
    assert!(source.allow_cidrs.is_empty());
    assert!(source.allow_protocols.is_empty());
}
