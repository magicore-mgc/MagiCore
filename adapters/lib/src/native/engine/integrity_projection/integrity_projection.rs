//! Integrity Projection owner.
//! Phụ trách integrity projection của native engine.

use base64::Engine as _;

use mgc_lockfile::EcosystemTag;
use mgc_resolver::protocols::ResolvedEntry;
use mgc_types::MgResult;

/// Project registry-verified integrity into the shared graph representation.
/// Chuyển integrity registry đã xác minh sang biểu diễn graph dùng chung.
pub(in crate::native::engine::engine) fn resolved_integrity(
    ecosystem: EcosystemTag,
    entry: &ResolvedEntry,
) -> MgResult<String> {
    if ecosystem != EcosystemTag::NuGet {
        return Ok(if entry.sha256.is_empty() {
            String::new()
        } else {
            format!("sha256-{}", entry.sha256)
        });
    }

    let hashes = entry
        .extra_markers
        .iter()
        .filter_map(|marker| marker.strip_prefix("sha512:"))
        .collect::<Vec<_>>();
    if hashes.len() != 1 {
        return Err(mgc_types::MgError::Integrity(format!(
            "NuGet resolution for {}@{} must contain exactly one SHA-512 registry marker",
            entry.name, entry.version
        )));
    }
    let digest = hashes[0];
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(digest)
        .map_err(|error| {
            mgc_types::MgError::Integrity(format!(
                "NuGet SHA-512 registry marker for {}@{} is not valid base64: {error}",
                entry.name, entry.version
            ))
        })?;
    if decoded.len() != 64 {
        return Err(mgc_types::MgError::Integrity(format!(
            "NuGet SHA-512 registry marker for {}@{} must decode to 64 bytes",
            entry.name, entry.version
        )));
    }
    Ok(format!("sha512-{digest}"))
}
