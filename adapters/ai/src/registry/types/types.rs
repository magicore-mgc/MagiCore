//! Registry identity and model metadata types.
//! Định nghĩa registry và metadata model.

use std::path::PathBuf;

/// Supported model registry identities.
/// Các registry model được nhận diện.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Registry {
    HuggingFace,
    TensorFlowHub,
    OnnxZoo,
    PyTorchHub,
    Local(PathBuf),
}

impl Registry {
    /// Classify a registry from a URL using the existing matching rules.
    /// Nhận diện registry theo các quy tắc URL hiện có.
    pub fn from_url(url: &str) -> Option<Self> {
        if url.contains("huggingface.co") {
            Some(Self::HuggingFace)
        } else if url.contains("tfhub.dev") {
            Some(Self::TensorFlowHub)
        } else if url.contains("onnx") || url.contains("github.com/onnx") {
            Some(Self::OnnxZoo)
        } else if url.contains("pytorch.org/hub") {
            Some(Self::PyTorchHub)
        } else if url.starts_with("file://") || url.starts_with('/') {
            Some(Self::Local(PathBuf::from(
                url.trim_start_matches("file://"),
            )))
        } else {
            None
        }
    }

    /// Return the configured base URL for a remote registry.
    /// Trả URL gốc đã cấu hình của registry từ xa.
    pub fn base_url(&self) -> Option<&str> {
        match self {
            Self::HuggingFace => Some("https://huggingface.co"),
            Self::TensorFlowHub => Some("https://tfhub.dev"),
            Self::OnnxZoo => Some("https://github.com/onnx/models"),
            Self::PyTorchHub => Some("https://pytorch.org/hub"),
            Self::Local(_) => None,
        }
    }

    /// Return the stable CLI/cache label for this registry.
    /// Trả tên ổn định dùng cho CLI/cache của registry.
    pub fn name(&self) -> &str {
        match self {
            Self::HuggingFace => "huggingface",
            Self::TensorFlowHub => "tfhub",
            Self::OnnxZoo => "onnx-zoo",
            Self::PyTorchHub => "pytorch-hub",
            Self::Local(_) => "local",
        }
    }
}

/// Metadata returned by a model registry query.
/// Metadata được trả về sau khi truy vấn registry.
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    pub id: String,
    pub registry: Registry,
    pub version: Option<String>,
    pub size_bytes: Option<u64>,
    pub format: Option<ModelFormat>,
    pub tags: Vec<String>,
}

/// Recognized model artifact formats.
/// Các định dạng artifact model được nhận diện.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelFormat {
    SafeTensors,
    PyTorch,
    TensorFlow,
    Onnx,
    HuggingFace,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
