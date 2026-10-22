#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Verify local artifact metadata and supported extensions.
//! Kiểm tra metadata artifact cục bộ và các extension hỗ trợ.

use crate::registry::ModelFormat;
use crate::registry::local_query::local_query::{detect_format, query_local};
use std::path::PathBuf;
use tempfile::TempDir;

fn temp_dir() -> TempDir {
    TempDir::new().unwrap()
}

#[tokio::test]
async fn query_local_returns_existing_file_metadata() {
    let temp = temp_dir();
    let model = temp.path().join("model.onnx");
    std::fs::write(&model, b"fake").unwrap();

    let metadata = query_local(temp.path(), "model.onnx").await.unwrap();
    assert_eq!(metadata.id, "model.onnx");
    assert_eq!(metadata.format, Some(ModelFormat::Onnx));
    assert_eq!(metadata.size_bytes, Some(4));
}

#[test]
fn detect_format_recognizes_supported_extensions() {
    assert_eq!(
        detect_format(&PathBuf::from("model.safetensors")),
        Some(ModelFormat::SafeTensors)
    );
    assert_eq!(
        detect_format(&PathBuf::from("model.onnx")),
        Some(ModelFormat::Onnx)
    );
    assert_eq!(
        detect_format(&PathBuf::from("model.pt")),
        Some(ModelFormat::PyTorch)
    );
    assert_eq!(
        detect_format(&PathBuf::from("model.pb")),
        Some(ModelFormat::TensorFlow)
    );
}
