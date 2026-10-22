//! Registry cache-root cases — kiểm tra mapping đường dẫn từng registry.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::cache_dir::cache_dir;
use crate::registry::Registry;
use std::path::PathBuf;

#[test]
fn maps_each_registry_to_its_established_cache_root() {
    let data = mgc_platform::paths::mgc_data_dir();
    let cases = [
        (Registry::HuggingFace, data.join("cache/ai/huggingface/hub")),
        (Registry::PyTorchHub, data.join("cache/ai/torch/hub")),
        (Registry::TensorFlowHub, data.join("cache/ai/tfhub")),
        (Registry::OnnxZoo, data.join("cache/ai/onnx")),
    ];

    for (registry, expected) in cases {
        assert_eq!(cache_dir(&registry).unwrap(), expected);
    }
}

#[test]
fn returns_local_cache_root_unchanged() {
    let local = PathBuf::from("/tmp/models");

    assert_eq!(cache_dir(&Registry::Local(local.clone())).unwrap(), local);
}
