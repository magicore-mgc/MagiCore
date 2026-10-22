//! Map each AI registry to its cache root — map registry AI sang cache root.

use crate::registry::Registry;
use mgc_types::MgResult;
use std::path::PathBuf;

/// Return the cache root selected for a registry.
/// Trả cache root được chọn cho registry.
pub fn cache_dir(registry: &Registry) -> MgResult<PathBuf> {
    let cache_path = match registry {
        Registry::HuggingFace => mgc_platform::paths::mgc_data_dir()
            .join("cache")
            .join("ai")
            .join("huggingface")
            .join("hub"),
        Registry::PyTorchHub => mgc_platform::paths::mgc_data_dir()
            .join("cache")
            .join("ai")
            .join("torch")
            .join("hub"),
        Registry::TensorFlowHub => mgc_platform::paths::mgc_data_dir()
            .join("cache")
            .join("ai")
            .join("tfhub"),
        Registry::OnnxZoo => mgc_platform::paths::mgc_data_dir()
            .join("cache")
            .join("ai")
            .join("onnx"),
        Registry::Local(path) => path.clone(),
    };

    Ok(cache_path)
}
