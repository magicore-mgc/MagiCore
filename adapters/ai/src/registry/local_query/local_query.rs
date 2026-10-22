//! Read metadata for a model artifact already present on disk.
//! Đọc metadata của model artifact đã có trên đĩa.

use crate::registry::{ModelFormat, ModelMetadata, Registry};
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Query a model path under the selected local registry root.
/// Truy vấn đường dẫn model trong thư mục registry cục bộ.
pub(in crate::registry) async fn query_local(
    path: &Path,
    model_id: &str,
) -> MgResult<ModelMetadata> {
    let model_path = path.join(model_id);
    if !model_path.exists() {
        return Err(MgError::Other(format!(
            "Local model not found: {}",
            model_path.display()
        )));
    }

    let size_bytes = std::fs::metadata(&model_path)
        .ok()
        .map(|metadata| metadata.len());

    Ok(ModelMetadata {
        id: model_id.to_string(),
        registry: Registry::Local(path.to_path_buf()),
        version: None,
        size_bytes,
        format: detect_format(&model_path),
        tags: vec![],
    })
}

/// Classify a model artifact by its file extension.
/// Phân loại model artifact theo phần mở rộng file.
fn detect_format(path: &Path) -> Option<ModelFormat> {
    path.extension()
        .and_then(|extension| extension.to_str())
        .and_then(|extension| match extension {
            "safetensors" => Some(ModelFormat::SafeTensors),
            "pt" | "pth" => Some(ModelFormat::PyTorch),
            "pb" | "h5" => Some(ModelFormat::TensorFlow),
            "onnx" => Some(ModelFormat::Onnx),
            _ => None,
        })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
