//! OCI descriptor model and builders
//! model và builder OCI descriptor

use serde::{Deserialize, Serialize};

/// OCI Descriptor (common for config, layers, blobs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciDescriptor {
    #[serde(rename = "mediaType", alias = "media_type")]
    pub media_type: String,
    pub size: i64,
    pub digest: String, // sha256:...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>, // optional inline data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
}

impl OciDescriptor {
    pub fn new(media_type: String, size: i64, digest: String) -> Self {
        Self {
            media_type,
            size,
            digest,
            data: None,
            urls: None,
            annotations: None,
        }
    }

    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_urls(mut self, urls: Vec<String>) -> Self {
        self.urls = Some(urls);
        self
    }

    pub fn with_annotations(
        mut self,
        annotations: std::collections::HashMap<String, String>,
    ) -> Self {
        self.annotations = Some(annotations);
        self
    }
}
