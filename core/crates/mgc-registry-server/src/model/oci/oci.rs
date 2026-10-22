//! OCI manifest, descriptor, and upload response records.
//! Các record manifest, descriptor và upload response OCI.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OciManifest {
    pub schema_version: i32,
    pub media_type: String,
    pub config: OciDescriptor,
    pub layers: Vec<OciDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Box<OciDescriptor>>,
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty", default)]
    pub annotations: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OciDescriptor {
    pub media_type: String,
    pub size: i64,
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OciBlobUploadResponse {
    pub location: String,
    pub range: Option<String>,
    pub docker_upload_uuid: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
