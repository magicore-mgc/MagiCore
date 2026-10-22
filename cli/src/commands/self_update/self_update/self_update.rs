//! Coordinate the verified release-to-install sequence.
//! Điều phối chuỗi từ release đã xác minh tới cài binary.

use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;

use crate::commands::self_update::{
    RELEASE_REPO, artifact_name, detect_variant, download_file, download_text, host_labels,
    resolve_provenance_mode, resolve_version, stage_binary, swap_binary, trust_roots,
    verify_checksum, verify_manifest,
};

/// Entry point for `mgc self-update`; verification precedes the binary swap.
/// Điểm vào `mgc self-update`; xác minh luôn xảy ra trước khi thay binary.
pub async fn run(
    version: Option<String>,
    variant: Option<String>,
    dry_run: bool,
    trust_root: Vec<String>,
    allow_unsigned: bool,
) -> Result<()> {
    // Explicit --variant wins; otherwise detect from the compiled feature
    // set (a wrong variant 404s, so never guess silently).
    // (--variant tường minh thắng; không thì detect lúc compile.)
    let package = match variant.as_deref() {
        Some("magicore-web") => "magicore-web".to_string(),
        Some("magicore") | Some("all") => "magicore".to_string(),
        Some(other) => return Err(SelfUpdateError::unsupported_variant(other)),
        None => detect_variant(),
    };
    let version_number = resolve_version(version.as_deref()).await?;
    let current = env!("CARGO_PKG_VERSION");
    if version_number == current && !dry_run {
        mgc_ui::info(&format!("already at mgc {current} — nothing to do"));
        return Ok(());
    }
    let (os, arch, ext) = host_labels()?;
    let archive = artifact_name(&package, &version_number, &os, &arch, &ext);
    let base_url = format!("https://github.com/{RELEASE_REPO}/releases/download/v{version_number}");
    let download_url = format!("{base_url}/{archive}");
    let checksum_url = format!("{download_url}.sha256");
    let manifest_url = format!("{base_url}/manifest.json");
    let manifest_sig_url = format!("{base_url}/manifest.json.sig");
    if dry_run {
        mgc_ui::info(&format!(
            "would download {download_url} (+ .sha256, + manifest.json)"
        ));
        return Ok(());
    }
    // Fail fast on provenance policy (bad flag combinations error before
    // any network, not after a half-finished update).
    // (Chốt policy provenance trước mọi download.)
    let roots = trust_roots(&trust_root);
    let mode = resolve_provenance_mode(&roots, allow_unsigned)?;
    mgc_ui::info(&format!("downloading mgc {version_number} ({archive})..."));
    let tmp = tempfile::tempdir().map_err(|e| anyhow::anyhow!("create temp dir: {e}"))?;
    let archive_path = tmp.path().join(&archive);
    download_file(&download_url, &archive_path).await?;
    let checksum_text = download_text(&checksum_url).await?;
    verify_checksum(&archive_path, &checksum_text)?;
    // Release-manifest provenance: every field must bind THIS exact
    // target, and the digest must match the bytes just verified. A
    // manifest for another version/variant/platform aborts here.
    // (Manifest phải bind đúng target này.)
    verify_manifest(
        &manifest_url,
        &manifest_sig_url,
        &version_number,
        &package,
        &os,
        &arch,
        &archive,
        &archive_path,
        &roots,
        mode,
    )
    .await?;
    let (staging_dir, staged) = stage_binary(&archive_path, &ext)?;
    let swap_result = swap_binary(&staged, &version_number).await;
    // Staging always cleaned (success AND failure) — keep() exists only
    // to outlive the extraction function, not the command.
    // (Staging luôn dọn — thành công hay thất bại.)
    let _ = std::fs::remove_dir_all(&staging_dir);
    swap_result?;
    mgc_ui::success(&format!("updated to mgc {version_number}"));
    Ok(())
}
