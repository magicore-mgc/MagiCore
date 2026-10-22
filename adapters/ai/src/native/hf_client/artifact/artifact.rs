//! Owns model-file and model-card download routes plus pure endpoint assembly.
//! Quản lý route tải file/card model và phần ghép endpoint thuần.

use mgc_types::MgResult;

use crate::native::hf_client::HuggingFaceClientError;

fn model_file_url(model_id: &str, filename: &str, revision: Option<&str>) -> String {
    format!(
        "https://huggingface.co/{}/{}/{}",
        model_id,
        revision.unwrap_or("main"),
        filename
    )
}

fn model_card_url(model_id: &str) -> String {
    format!("https://huggingface.co/{}/raw/main/README.md", model_id)
}

/// Download model file từ HuggingFace Hub → Vec<u8> (dùng chung http_download).
// (Download a model file from the Hub into memory via the shared http_download.)
pub async fn download_file(
    model_id: &str,
    filename: &str,
    revision: Option<&str>,
    token: Option<&str>,
) -> MgResult<Vec<u8>> {
    let url = model_file_url(model_id, filename, revision);
    let mut req = reqwest::Client::new().get(&url);
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }
    let response = req
        .send()
        .await
        .map_err(|error| HuggingFaceClientError::request("HuggingFace download", error))?;
    if !response.status().is_success() {
        return Err(HuggingFaceClientError::http_status("HuggingFace", response.status()).into());
    }
    // ponytail: full-buffer — file model lớn cần stream → dùng install::download_model (P3 resume)
    // (ponytail: full-buffer — large models should use install::download_model; P3 adds streaming/resume)
    let bytes = response
        .bytes()
        .await
        .map_err(|error| HuggingFaceClientError::request("download body", error))?;
    Ok(bytes.to_vec())
}

/// Get model card (README.md) từ HuggingFace
/// Lấy thẻ model (README.md) từ HuggingFace.
pub async fn get_model_card(model_id: &str) -> MgResult<String> {
    let url = model_card_url(model_id);
    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|error| HuggingFaceClientError::request("model card request", error))?;
    if !response.status().is_success() {
        return Err(HuggingFaceClientError::http_status("model card", response.status()).into());
    }
    Ok(response
        .text()
        .await
        .map_err(|error| HuggingFaceClientError::request("model card body", error))?)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
