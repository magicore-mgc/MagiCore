#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Regression tests for registry identity mapping.
//! Test hồi quy cho ánh xạ identity registry.

use crate::registry::{ModelFormat, ModelMetadata, Registry};
use std::path::PathBuf;

#[test]
fn registry_from_url_keeps_existing_provider_rules() {
    assert_eq!(
        Registry::from_url("https://huggingface.co/bert-base"),
        Some(Registry::HuggingFace)
    );
    assert_eq!(
        Registry::from_url("https://tfhub.dev/google/model/1"),
        Some(Registry::TensorFlowHub)
    );
    assert_eq!(
        Registry::from_url("https://github.com/onnx/models"),
        Some(Registry::OnnxZoo)
    );
    assert_eq!(
        Registry::from_url("file:///models"),
        Some(Registry::Local(PathBuf::from("/models")))
    );
}

#[test]
fn registry_base_urls_preserve_local_none() {
    assert_eq!(
        Registry::HuggingFace.base_url(),
        Some("https://huggingface.co")
    );
    assert_eq!(
        Registry::TensorFlowHub.base_url(),
        Some("https://tfhub.dev")
    );
    assert_eq!(Registry::Local(PathBuf::from("/tmp")).base_url(), None);
}

#[test]
fn registry_names_remain_stable() {
    assert_eq!(Registry::HuggingFace.name(), "huggingface");
    assert_eq!(Registry::TensorFlowHub.name(), "tfhub");
    assert_eq!(Registry::OnnxZoo.name(), "onnx-zoo");
}

#[test]
fn metadata_types_keep_public_fields_constructible() {
    let metadata = ModelMetadata {
        id: "model".into(),
        registry: Registry::HuggingFace,
        version: None,
        size_bytes: Some(1),
        format: Some(ModelFormat::HuggingFace),
        tags: vec!["text-generation".into()],
    };
    assert_eq!(metadata.id, "model");
    assert_eq!(metadata.size_bytes, Some(1));
}
