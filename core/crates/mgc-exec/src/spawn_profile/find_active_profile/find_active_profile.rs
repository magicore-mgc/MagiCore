use std::path::Path;

use crate::policy::LaneProfile;
use crate::spawn_profile::{
    ProfileLookupError, SPAWN_PROFILE_SCHEMA, SpawnProfileFile, profile_file_name,
};

/// Load a profile for an exact tuple and require its lock digest to match.
/// Returns `None` when no profile exists (lane stays level-2 tripwire-only).
/// Nạp profile cho đúng tuple và yêu cầu khớp lock digest; thiếu profile trả `None`.
pub fn find_active_profile(
    store_dir: &Path,
    framework: &str,
    framework_version: &str,
    os: &str,
    operation: &str,
    lock_digest: Option<&str>,
) -> Result<Option<LaneProfile>, ProfileLookupError> {
    let path = store_dir.join(profile_file_name(framework, operation, os));
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(ProfileLookupError::Invalid(error.to_string())),
    };
    let file: SpawnProfileFile = serde_json::from_slice(&bytes)
        .map_err(|error| ProfileLookupError::Invalid(error.to_string()))?;
    if file.schema_version != SPAWN_PROFILE_SCHEMA {
        return Err(ProfileLookupError::Invalid(format!(
            "unknown schema {}",
            file.schema_version
        )));
    }
    // Exact version pin: a range match would need per-range evidence.
    if file.profile.framework != framework
        || file.profile.version != framework_version
        || file.profile.os != os
        || file.profile.operation != operation
    {
        return Ok(None);
    }
    // Lock drift expires the profile (Spec rev4 §6).
    if file.lock_digest.as_deref() != lock_digest {
        return Ok(None);
    }
    Ok(Some(file.profile))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
