//! Fetch and resolve one immutable model revision.
//! Tải và resolve một revision model bất biến.

use mgc_types::{MgError, MgResult};
use serde_json::Value;

use super::super::hfhub::HfHubProtocol;
use super::super::model_identity::{check_model_id, check_revision};
use super::super::model_resolution::ModelResolution;
use super::super::model_tree::parse_model_tree;

impl HfHubProtocol {
    /// Resolve a model revision to its locked file list via the tree API.
    /// Resolve revision model thành danh sách file đã lock qua tree API.
    pub async fn resolve_model(&self, model: &str, revision: &str) -> MgResult<ModelResolution> {
        check_model_id(model).map_err(MgError::from)?;
        check_revision(revision).map_err(MgError::from)?;
        let url = format!(
            "{}/api/models/{model}/tree/{revision}?recursive=true",
            self.base_url
        );
        let response = self
            .client
            .get(&url)
            .await
            .map_err(|error| MgError::Network(format!("GET {url} failed: {error}")))?;
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|error| MgError::Network(format!("read {url} body failed: {error}")))?;
        if !status.is_success() {
            return Err(MgError::Network(format!(
                "GET {url} returned {status} — model tree required (fail-closed)"
            )));
        }
        let entries: Vec<Value> = serde_json::from_str(&body)
            .map_err(|error| MgError::Other(format!("parse model tree failed: {error}")))?;
        let files = parse_model_tree(&entries, model, revision, &self.base_url)?;
        Ok(ModelResolution {
            model: model.to_string(),
            commit: revision.to_string(),
            files,
        })
    }
}
