#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Verify existing model URL and path parser behavior.
//! Kiểm tra hành vi parser URL và đường dẫn model hiện có.

use crate::registry::parser::{
    parse_hf_id, parse_onnx_path as parse_onnx_model_path, parse_tfhub_url as parse_tfhub_model_url,
};

#[test]
fn parse_hf_id_simple() {
    let (repository, revision) = parse_hf_id("openai/gpt-2").unwrap();
    assert_eq!(repository, "openai/gpt-2");
    assert_eq!(revision, None);
}

#[test]
fn parse_hf_id_with_url() {
    let (repository, _) = parse_hf_id("https://huggingface.co/bert-base-uncased/bert").unwrap();
    assert_eq!(repository, "bert-base-uncased/bert");
}

#[test]
fn parse_hf_id_invalid() {
    assert!(parse_hf_id("invalid").is_err());
}

#[test]
fn parse_tfhub_url() {
    let (model, version) = parse_tfhub_model_url("https://tfhub.dev/google/imagenet/1").unwrap();
    assert_eq!(model, "google/imagenet");
    assert_eq!(version, "1");
}

#[test]
fn parse_tfhub_url_without_https() {
    let (model, version) = parse_tfhub_model_url("tfhub.dev/google/bert/2").unwrap();
    assert_eq!(model, "google/bert");
    assert_eq!(version, "2");
}

#[test]
fn parse_onnx_path() {
    let path = parse_onnx_model_path("vision/resnet50.onnx").unwrap();
    assert_eq!(path, "vision/resnet50.onnx");
}

#[test]
fn parse_onnx_path_invalid() {
    assert!(parse_onnx_model_path("invalid-path").is_err());
}
