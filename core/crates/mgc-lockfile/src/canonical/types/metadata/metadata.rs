//! Owns the public v4 metadata type.
//! Sở hữu type metadata v4 công khai.

use crate::v4::SignatureBlock;
use serde::{Deserialize, Serialize};
/// v4 metadata — Metadata v4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockfileV4Metadata {
    /// Generation timestamp, ISO-8601 (OUT-of-payload: two runs over the
    /// same graph must digest identically).
    pub generated_at: String,
    /// Generator id (`mgc/1.2.0`) — IN-payload.
    pub generator: String,
    /// Digest of the canonical payload (`blake3-<base64>`, OUT).
    pub lockfile_hash: String,
    /// Inline signature (OUT) — Chữ ký nhúng.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<SignatureBlock>,
}
