#![allow(clippy::unwrap_used)]

// Offline tests for model artifact endpoint construction.
// Test offline cho việc dựng endpoint artifact model.
use super::super::*;

#[test]
fn model_file_url_uses_main_revision_when_omitted() {
    assert_eq!(
        model_file_url("org/model", "config.json", None),
        "https://huggingface.co/org/model/main/config.json"
    );
}

#[test]
fn model_file_url_and_card_url_preserve_explicit_paths() {
    assert_eq!(
        model_file_url("org/model", "weights/model.safetensors", Some("v1")),
        "https://huggingface.co/org/model/v1/weights/model.safetensors"
    );
    assert_eq!(
        model_card_url("org/model"),
        "https://huggingface.co/org/model/raw/main/README.md"
    );
}
