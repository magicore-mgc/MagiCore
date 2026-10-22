//! Implements the existing native metadata/list/existence API over the shared HF client.
//! Cài đặt API metadata/list/exists hiện tại trên client Hugging Face dùng chung.

use crate::native::hf_client::{
    HuggingFaceClient, HuggingFaceClientError, parse_model_ids, parse_model_info,
};
use crate::native::{ApiClient, ModelInfo};
use mgc_types::MgResult;

#[async_trait::async_trait]
impl ApiClient for HuggingFaceClient {
    async fn get_model_info(&self, model_id: &str) -> MgResult<ModelInfo> {
        if model_id.trim().is_empty() {
            return Err(HuggingFaceClientError::EmptyModelId.into());
        }
        let json = self.get_json(&format!("/models/{model_id}")).await?;
        Ok(parse_model_info(&json, model_id))
    }

    async fn list_models(&self, limit: Option<usize>) -> MgResult<Vec<String>> {
        let limit = limit.unwrap_or(10);
        let json = self.get_json(&format!("/models?limit={limit}")).await?;
        Ok(parse_model_ids(&json))
    }

    async fn model_exists(&self, model_id: &str) -> MgResult<bool> {
        if model_id.trim().is_empty() {
            return Ok(false);
        }
        let url = format!("{}/models/{model_id}", self.api_url);
        let mut req = reqwest::Client::new().head(&url);
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }
        let response = req
            .send()
            .await
            .map_err(|error| HuggingFaceClientError::request("HuggingFace HEAD", error))?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
