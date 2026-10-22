//! Parse model identifiers and paths for supported registry formats.
//! Phân tích định danh và đường dẫn model theo registry hỗ trợ.

use mgc_types::{MgError, MgResult};

/// Parse a Hugging Face URL or `org/model` identifier.
/// Accepted examples: `org/model` or `https://huggingface.co/org/model`.
/// Phân tích URL Hugging Face hoặc định danh `org/model`.
pub fn parse_hf_id(input: &str) -> MgResult<(String, Option<String>)> {
    let clean = input
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("huggingface.co/");

    let parts: Vec<&str> = clean.split('/').collect();

    if parts.len() >= 2 {
        let repo_id = format!("{}/{}", parts[0], parts[1]);
        let revision = parts.get(3).map(|part| part.to_string());
        Ok((repo_id, revision))
    } else {
        Err(MgError::Other(format!("Invalid HuggingFace ID: {}", input)))
    }
}

/// Parse a TensorFlow Hub model URL.
/// Example: `https://tfhub.dev/google/model/version`.
/// Phân tích URL model TensorFlow Hub.
pub fn parse_tfhub_url(url: &str) -> MgResult<(String, String)> {
    let clean = url
        .trim_start_matches("https://")
        .trim_start_matches("tfhub.dev/");

    let parts: Vec<&str> = clean.split('/').collect();

    if parts.len() >= 3 {
        let model_id = parts[..parts.len() - 1].join("/");
        let version = parts[parts.len() - 1].to_string();
        Ok((model_id, version))
    } else {
        Err(MgError::Other(format!("Invalid TFHub URL: {}", url)))
    }
}

/// Accept an ONNX model path when it contains the `.onnx` marker.
/// Example: `vision/classification/resnet/model.onnx`.
/// Nhận đường dẫn ONNX nếu có marker `.onnx`.
pub fn parse_onnx_path(path: &str) -> MgResult<String> {
    if path.contains(".onnx") {
        Ok(path.to_string())
    } else {
        Err(MgError::Other(format!("Invalid ONNX path: {}", path)))
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
