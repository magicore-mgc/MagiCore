#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Verify Hub metadata fields and missing-field defaults.
//! Kiểm tra trường metadata Hub và giá trị mặc định khi thiếu.

use crate::registry::{ModelFormat, Registry, metadata_from_hf_json::metadata_from_hf_json};

#[test]
fn maps_model_revision_tags_and_format() {
    let json: serde_json::Value = serde_json::from_str(
        r#"{"id":"gpt2","sha":"e7da7f2","tags":["transformers","text-generation"]}"#,
    )
    .unwrap();

    let metadata = metadata_from_hf_json(&json, "fallback");
    assert_eq!(metadata.id, "gpt2");
    assert_eq!(metadata.registry, Registry::HuggingFace);
    assert_eq!(metadata.version.as_deref(), Some("e7da7f2"));
    assert_eq!(metadata.tags.len(), 2);
    assert_eq!(metadata.format, Some(ModelFormat::HuggingFace));
}

#[test]
fn missing_fields_keep_fallback_id_main_revision_and_no_tags() {
    let metadata = metadata_from_hf_json(&serde_json::Value::Null, "mymodel");
    assert_eq!(metadata.id, "mymodel");
    assert_eq!(metadata.version.as_deref(), Some("main"));
    assert!(metadata.tags.is_empty());
}
