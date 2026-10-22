//! Owns client configuration, builder methods, JSON transport and redacted Debug.
//! Quản lý cấu hình, builder, JSON transport và Debug đã che thông tin nhạy cảm.

use mgc_types::MgResult;
use serde_json::Value;

use super::super::HuggingFaceClientError;

#[derive(Clone)]
pub struct HuggingFaceClient {
    pub api_url: String,
    pub token: Option<String>,
}

impl std::fmt::Debug for HuggingFaceClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HuggingFaceClient")
            .field("token_configured", &self.token.is_some())
            .finish()
    }
}

impl HuggingFaceClient {
    pub fn new() -> Self {
        HuggingFaceClient {
            api_url: "https://huggingface.co/api".to_string(),
            token: None,
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn with_api_url(mut self, url: String) -> Self {
        self.api_url = url;
        self
    }

    /// GET JSON từ endpoint — gắn Bearer token nếu có, kiểm tra status.
    /// pub(crate): dùng chung cho registry query (một đường HTTP duy nhất cho HF).
    // (GET JSON from endpoint — Bearer token when present, status checked.
    // crate-visible so registry queries share the single HF HTTP path.)
    pub(crate) async fn get_json(&self, path: &str) -> MgResult<Value> {
        self.get_json_typed(path).await.map_err(Into::into)
    }

    async fn get_json_typed(&self, path: &str) -> Result<Value, HuggingFaceClientError> {
        let url = format!("{}{}", self.api_url, path);
        let mut req = reqwest::Client::new().get(&url);
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }
        let response = req
            .send()
            .await
            .map_err(|error| HuggingFaceClientError::request("HuggingFace request", error))?;
        if !response.status().is_success() {
            return Err(HuggingFaceClientError::http_status(
                "HuggingFace",
                response.status(),
            ));
        }
        response
            .json()
            .await
            .map_err(HuggingFaceClientError::json_parse)
    }
}

impl Default for HuggingFaceClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
