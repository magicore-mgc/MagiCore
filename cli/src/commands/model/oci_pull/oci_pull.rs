// oci_pull owner — oci_pull owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::cas_pull::cas_import;
use super::super::manifest_io::now_iso;
use super::super::registry_client::client;
use super::OciPullError;
use anyhow::{Context, Result};
/// Pull an OCI manifest and its layers into CAS.
/// oci://registry/repo:tag → pull manifest + layers → CAS (P1 cơ bản).
pub(in crate::commands::model) async fn pull_oci(
    store: &mgc_store::cas::ContentStore,
    oci: &str,
) -> Result<(String, Vec<String>, u64)> {
    let (registry, rest) = oci
        .split_once('/')
        .ok_or_else(|| OciPullError::InvalidSource(oci.to_string()))?;
    let (repo, tag) = match rest.rsplit_once(':') {
        Some((r, t)) if !r.is_empty() && !t.is_empty() => (r, t),
        _ => (rest, "latest"),
    };
    let base = if registry.contains("://") {
        registry.to_string()
    } else {
        format!("http://{registry}")
    };

    let c = client(&base, None)?;
    let manifest = c
        .pull_manifest(repo, tag)
        .await
        .with_context(|| OciPullError::ManifestPullFailed)?;

    let mut blobs = Vec::new();
    let mut total = 0u64;
    for layer in &manifest.layers {
        let data = c
            .pull_blob(repo, &layer.digest)
            .await
            .with_context(|| format!("pull blob {}", layer.digest))?;
        let tmp =
            std::env::temp_dir().join(format!("mgc-oci-{}-{}", std::process::id(), now_iso()));
        std::fs::write(&tmp, &data)?;
        let (hash, len) = cas_import(store, &tmp)?;
        let _ = std::fs::remove_file(&tmp);
        blobs.push(hash);
        total += len;
        println!(
            "  blob {} ({len} bytes)",
            layer.digest.trim_start_matches("sha256:")
        );
    }

    let name = format!("{repo}:{tag}");
    Ok((name, blobs, total))
}
