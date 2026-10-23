//! API client contract and model metadata DTO.
//! Hợp đồng client API và DTO metadata model.

use mgc_types::MgResult;

/// Generic API client trait
#[async_trait::async_trait]
pub trait ApiClient {
    /// Fetch model metadata from registry
    async fn get_model_info(&self, model_id: &str) -> MgResult<ModelInfo>;

    /// List available models (optional pagination)
    async fn list_models(&self, limit: Option<usize>) -> MgResult<Vec<String>>;

    /// Check if model exists
    async fn model_exists(&self, model_id: &str) -> MgResult<bool>;
}

/// Model information từ API
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub author: Option<String>,
    pub downloads: Option<u64>,
    pub tags: Vec<String>,
    pub pipeline_tag: Option<String>,
    pub size_bytes: Option<u64>,
}

impl ModelInfo {
    pub fn new(id: &str) -> Self {
        ModelInfo {
            id: id.to_string(),
            author: None,
            downloads: None,
            tags: vec![],
            pipeline_tag: None,
            size_bytes: None,
        }
    }
}
