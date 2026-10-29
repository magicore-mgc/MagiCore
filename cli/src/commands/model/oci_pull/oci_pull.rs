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
    // PAR-12: an unpinned tag (`latest`) is a moving target and the manifest
    // digest is self-declared, so a model pulled by tag cannot be reproduced.
    // A digest reference is accepted as-is; a tag is only accepted when it is
    // explicit, and the resolved digest is returned so the caller can record it.
    // (PAR-12: tag không pin (`latest`) là đích di động và digest trong
    // manifest do chính nó khai, nên model pull theo tag không tái lập được.
    // Tham chiếu digest được nhận nguyên; tag chỉ được nhận khi nêu tường minh,
    // và digest đã resolve được trả về để caller ghi lại.)
    let (repo, reference) = match rest.rsplit_once('@') {
        Some((r, d)) if !r.is_empty() && !d.is_empty() => (r, d),
        _ => match rest.rsplit_once(':') {
            Some((r, t)) if !r.is_empty() && !t.is_empty() => (r, t),
            _ => {
                return Err(OciPullError::UnpinnedReference(rest.to_string()).into());
            }
        },
    };
    // PAR-12: plaintext HTTP is only acceptable for a loopback registry; any
    // other registry must be addressed over TLS.
    // (PAR-12: HTTP trần chỉ chấp nhận cho registry loopback; registry khác phải
    // dùng TLS.)
    let base = if registry.contains("://") {
        registry.to_string()
    } else if is_loopback_registry(registry) {
        format!("http://{registry}")
    } else {
        format!("https://{registry}")
    };

    let c = client(&base, None)?;
    let manifest = c
        .pull_manifest(repo, reference)
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

    let name = format!("{repo}:{reference}");
    Ok((name, blobs, total))
}

/// Whether a registry host is a loopback address (plaintext is acceptable).
/// Host registry có phải loopback (chấp nhận plaintext) hay không.
pub(super) fn is_loopback_registry(registry: &str) -> bool {
    // A bracketed IPv6 literal keeps its colons, so it must be matched before
    // the port split; otherwise `[::1]:5000` reads as host `[`.
    // (IPv6 trong ngoặc giữ dấu hai chấm nên phải so trước khi tách port; nếu
    // không `[::1]:5000` bị đọc thành host `[`.)
    let host = if let Some(rest) = registry.strip_prefix('[') {
        rest.split(']').next().map(|inner| format!("[{inner}]"))
    } else {
        None
    }
    .unwrap_or_else(|| {
        registry
            .rsplit_once(':')
            .filter(|(_, port)| port.chars().all(|c| c.is_ascii_digit()))
            .map(|(host, _)| host)
            .unwrap_or(registry)
            .to_string()
    });
    matches!(host.as_str(), "127.0.0.1" | "localhost" | "::1" | "[::1]")
}

#[cfg(test)]
#[path = "test/oci_pull.rs"]
mod test;
