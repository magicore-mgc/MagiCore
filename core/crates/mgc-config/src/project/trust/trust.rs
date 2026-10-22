//! Accepted signing key identifiers for project lock verification.
//! Key ID được chấp nhận để verify lock của project.

use serde::{Deserialize, Serialize};

/// Trust roots — `mgc.toml [trust]` key ids for required lock signatures.
/// Trust root `[trust]` chứa key ID cho chữ ký lock bắt buộc.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustConfig {
    /// Accepted signing key ids (8-byte BLAKE3 hex) — key ID ký được chấp nhận.
    #[serde(default)]
    pub keys: Vec<String>,
}
