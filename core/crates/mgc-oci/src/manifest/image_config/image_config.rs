//! OCI image configuration models
//! model cấu hình OCI image

use serde::{Deserialize, Serialize};

/// OCI Image Config (for model metadata)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciImageConfig {
    pub created: String,
    pub architecture: String,
    pub os: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OciConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<OciRootFs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<OciHistory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciConfig {
    #[serde(
        rename = "User",
        alias = "user",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<String>,
    #[serde(
        rename = "ExposedPorts",
        alias = "exposed_ports",
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_ports: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "Env", alias = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(
        rename = "Entrypoint",
        alias = "entrypoint",
        skip_serializing_if = "Option::is_none"
    )]
    pub entrypoint: Option<Vec<String>>,
    #[serde(rename = "Cmd", alias = "cmd", skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    #[serde(
        rename = "Volumes",
        alias = "volumes",
        skip_serializing_if = "Option::is_none"
    )]
    pub volumes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "WorkingDir",
        alias = "working_dir",
        skip_serializing_if = "Option::is_none"
    )]
    pub working_dir: Option<String>,
    #[serde(
        rename = "Labels",
        alias = "labels",
        skip_serializing_if = "Option::is_none"
    )]
    pub labels: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciRootFs {
    pub r#type: String, // "layers"
    #[serde(alias = "diffIds")]
    pub diff_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciHistory {
    pub created: String,
    #[serde(alias = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(alias = "emptyLayer", skip_serializing_if = "Option::is_none")]
    pub empty_layer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
