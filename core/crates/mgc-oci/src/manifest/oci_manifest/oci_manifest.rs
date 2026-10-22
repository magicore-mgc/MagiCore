//! OCI image manifest model
//! model OCI image manifest

use crate::manifest::OciDescriptor;
use serde::{Deserialize, Serialize};

/// OCI Image Manifest v1 (schemaVersion 2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciManifest {
    #[serde(rename = "schemaVersion", alias = "schema_version")]
    pub schema_version: i32,
    #[serde(rename = "mediaType", alias = "media_type")]
    pub media_type: String,
    pub config: OciDescriptor,
    pub layers: Vec<OciDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Box<OciDescriptor>>,
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty", default)]
    pub annotations: std::collections::HashMap<String, String>,
}

impl OciManifest {
    pub fn new(config: OciDescriptor, layers: Vec<OciDescriptor>) -> Self {
        Self {
            schema_version: 2,
            media_type: "application/vnd.oci.image.manifest.v1+json".to_string(),
            config,
            layers,
            subject: None,
            annotations: std::collections::HashMap::new(),
        }
    }
}
