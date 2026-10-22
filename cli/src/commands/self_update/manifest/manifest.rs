//! Bind archive coordinates, size and digest to the release manifest.
//! Ràng buộc tọa độ archive, size và digest với manifest release.

use super::signature::verify_manifest_signature;
use crate::commands::self_update::error::SelfUpdateError;
use crate::commands::self_update::{
    MAX_DOC_BYTES, ProvenanceMode, ReleaseManifest, download_capped, sha256_file_streaming,
};
use anyhow::Result;
/// Fetch + verify the release manifest, then bind it to this exact
/// download. Every field must match the requested target; the digest
/// must match the verified bytes; the size must match too (truncation
/// check independent of the streaming quota).
/// (Fetch + verify manifest — mọi field phải khớp target này.)
#[allow(clippy::too_many_arguments)]
pub(in crate::commands) async fn verify_manifest(
    manifest_url: &str,
    manifest_sig_url: &str,
    version_number: &str,
    package: &str,
    os: &str,
    arch: &str,
    archive: &str,
    archive_path: &std::path::Path,
    trust_root: &[String],
    mode: ProvenanceMode,
) -> Result<()> {
    let manifest_bytes = match download_capped(manifest_url, MAX_DOC_BYTES).await {
        Ok(bytes) => bytes,
        Err(e) => {
            // P0/F2: a manifest fetch failure is NEVER silently treated
            // as "legacy release" — DNS/TLS/500 and 404 all land here.
            // Verified mode fails closed; only the explicit unsigned
            // opt-in (no roots) proceeds sha256-only, LOUDLY.
            // (Lỗi tải manifest không bao giờ âm thầm thành "release cũ".)
            if mode == ProvenanceMode::Verified {
                return Err(SelfUpdateError::manifest_invalid(&format!(
                    "manifest download failed under configured trust roots: {e}"
                )));
            }
            mgc_ui::warning(
                "no release manifest published for this version — provenance UNVERIFIED (sha256 transport check only; explicit --allow-unsigned)",
            );
            return Ok(());
        }
    };
    let manifest_text = String::from_utf8(manifest_bytes.clone())
        .map_err(|e| SelfUpdateError::manifest_invalid(&format!("manifest is not UTF-8: {e}")))?;
    let manifest: ReleaseManifest = serde_json::from_str(&manifest_text)
        .map_err(|e| SelfUpdateError::manifest_invalid(&e.to_string()))?;
    // The manifest must be FOR this release; then select OUR entry.
    // (Manifest phải đúng release này; rồi chọn entry của target.)
    if manifest.version != version_number {
        return Err(SelfUpdateError::manifest_invalid(&format!(
            "manifest version {} != {version_number}",
            manifest.version
        )));
    }
    let entry = manifest
        .artifacts
        .iter()
        .find(|entry| {
            entry.package == package
                && entry.os == os
                && entry.arch == arch
                && entry.archive == archive
        })
        .ok_or_else(|| {
            SelfUpdateError::manifest_invalid(&format!(
                "no manifest entry binds {archive} (manifest covers {} artifact(s))",
                manifest.artifacts.len()
            ))
        })?;
    // Digest + size bind the manifest entry to the exact bytes on disk
    // (streaming: size and digest in one pass, no full RAM buffering).
    // (Digest + size bind entry với byte đã tải — stream một lượt.)
    let (actual_size, actual_digest) = sha256_file_streaming(archive_path)?;
    if actual_size != entry.size {
        return Err(SelfUpdateError::manifest_invalid(&format!(
            "size {actual_size} != manifest {}",
            entry.size
        )));
    }
    if actual_digest != entry.sha256.to_lowercase() {
        return Err(SelfUpdateError::manifest_invalid(
            "digest mismatch between manifest and downloaded bytes",
        ));
    }
    verify_manifest_signature(manifest_sig_url, &manifest_bytes, trust_root, mode).await
}
