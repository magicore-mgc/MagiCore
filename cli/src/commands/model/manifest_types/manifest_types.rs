// manifest_types owner — manifest_types owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(in crate::commands::model) struct ModelManifest {
    pub(in crate::commands::model) name: String,
    pub(in crate::commands::model) source: String,
    pub(in crate::commands::model) blobs: Vec<String>,
    pub(in crate::commands::model) total_bytes: u64,
    pub(in crate::commands::model) pulled_at: String,
}
