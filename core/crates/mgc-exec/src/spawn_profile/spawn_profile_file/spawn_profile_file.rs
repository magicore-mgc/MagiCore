use serde::{Deserialize, Serialize};

use crate::policy::LaneProfile;

/// Stored profile with capture metadata — profile lưu kèm metadata capture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnProfileFile {
    /// Schema version — must equal [`crate::spawn_profile::SPAWN_PROFILE_SCHEMA`].
    /// Phiên bản schema — phải bằng hằng schema.
    pub schema_version: String,
    /// MagiCore version that captured this profile.
    /// Version MagiCore đã capture profile này.
    pub mgc_version: String,
    /// Corpus snapshot id the capture belongs to (empty = ad-hoc).
    /// Id snapshot corpus của lần capture (rỗng = ad-hoc).
    #[serde(default)]
    pub corpus_id: String,
    /// Digest of the `mgc.lock` the capture ran against; a different lock
    /// means re-verification (Spec rev4 §6).
    /// Digest của `mgc.lock` lúc capture; lock khác nghĩa là re-verify.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_digest: Option<String>,
    /// Unix seconds when captured — giây unix lúc capture.
    pub captured_at: i64,
    /// The observed lane profile — profile lane đã quan sát.
    pub profile: LaneProfile,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
