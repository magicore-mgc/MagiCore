//! Previous signing-key selection / Chọn key ký trước đó.

use std::path::Path;

use crate::attestation::{Attestation, PreviousKey};

/// Previous signing key for rotation continuity, if the stored
/// attestation names a different key — `None` on first attest,
/// unparseable slots, or same-key rotation.
/// (Key ký cũ để rotation liên tục — `None` khi attest đầu, slot hỏng,
/// hoặc cùng key.)
pub(in crate::attestation) fn previous_key_for(
    path: &Path,
    current_key_id: &str,
) -> Option<PreviousKey> {
    let text = std::fs::read_to_string(path).ok()?;
    let old: Attestation = serde_json::from_str(&text).ok()?;
    (old.key_id != current_key_id).then_some(PreviousKey {
        key_id: old.key_id,
        public_key: old.public_key,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
