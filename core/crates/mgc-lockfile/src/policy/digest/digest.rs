//! Decode canonical lockfile BLAKE3 digest bytes.
//! Giải mã byte digest BLAKE3 canonical của lockfile.

use crate::{LockfileError, LockfileResult};

/// Decode a `blake3-<base64>` digest to raw bytes.
/// Giải digest `blake3-<base64>` ra byte thô.
pub(in crate::policy) fn blake3_digest_bytes(digest: &str) -> LockfileResult<Vec<u8>> {
    use base64::Engine;
    let raw = digest
        .strip_prefix("blake3-")
        .ok_or_else(|| LockfileError::VerificationFailed(format!("malformed digest '{digest}'")))?;
    base64::engine::general_purpose::STANDARD
        .decode(raw)
        .map_err(|e| LockfileError::VerificationFailed(format!("malformed digest: {e}")))
}
