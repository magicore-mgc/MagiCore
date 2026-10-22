#![allow(clippy::unwrap_used)]

// Offline tests for JSON-to-model metadata projection.
// Test offline cho phép chiếu JSON sang metadata model.
use super::super::*;

#[test]
fn parses_full_model_info() {
    let json: serde_json::Value = serde_json::from_str(
        r#"{
            "id": "bert-base-uncased",
            "author": "google-bert",
            "downloads": 4000000,
            "tags": ["transformers", "pytorch"],
            "pipeline_tag": "fill-mask"
        }"#,
    )
    .unwrap();
    let info = parse_model_info(&json, "fallback");
    assert_eq!(info.id, "bert-base-uncased");
    assert_eq!(info.author.as_deref(), Some("google-bert"));
    assert_eq!(info.downloads, Some(4_000_000));
    assert_eq!(info.tags.len(), 2);
    assert_eq!(info.pipeline_tag.as_deref(), Some("fill-mask"));
}

#[test]
fn parses_minimal_model_info_with_fallback_id() {
    let json: serde_json::Value = serde_json::from_str("{}").unwrap();
    let info = parse_model_info(&json, "gpt2");
    assert_eq!(
        info.id, "gpt2",
        "missing id must use the fallback without panicking"
    );
    assert_eq!(info.author, None);
    assert!(info.tags.is_empty());
}

#[test]
fn parses_model_ids_and_skips_entries_without_id() {
    let json: serde_json::Value =
        serde_json::from_str(r#"[{"id":"a"},{"nope":1},{"id":"b"}]"#).unwrap();
    assert_eq!(
        parse_model_ids(&json),
        vec!["a".to_string(), "b".to_string()]
    );
    assert!(parse_model_ids(&serde_json::Value::Null).is_empty());
}
