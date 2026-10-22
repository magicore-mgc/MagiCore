//! Trusted publishing data contracts.
//! Contract dữ liệu trusted publishing.

use super::super::prelude::*;

#[derive(Debug, Clone)]
pub struct TrustedPublishingConfig {
    pub issuers: Vec<String>,
    pub audience: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuilderIdentity {
    pub issuer: String,
    pub subject: String,
    pub repository: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_workflow_ref: Option<String>,
    pub workflow_ref: Option<String>,
    pub event_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttestationPayload {
    pub schema_version: u32,
    pub package: String,
    pub version: String,
    pub digest: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_sha256: Option<String>,
    pub sequence: u64,
    pub builder: BuilderIdentity,
    pub created_at_unix: u64,
    pub previous_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedAttestation {
    pub payload: AttestationPayload,
    pub entry_hash: String,
    pub key_id: String,
    pub signature: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigstore_bundle: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttestationBundle {
    pub package: String,
    pub public_key: String,
    pub key_id: String,
    pub head_sequence: u64,
    pub entries: Vec<SignedAttestation>,
}
