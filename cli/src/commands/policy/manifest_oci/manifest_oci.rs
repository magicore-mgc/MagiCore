//! Parse OCI references and retrieve signed policy artifacts.
//! Phân tích OCI reference và tải artifact policy đã ký.

use std::path::Path;

use super::super::manifest_distribution::error::{
    PolicyManifestDistributionError as DistributionError, PolicyManifestDistributionResult,
};

/// OCI-registry distribution (Spec rev4 follow-up — decided): the policy
/// document travels as an OCI config blob so lanes can pin it by digest or
/// tag without a new registry protocol. Layers must be empty — anything
/// else is refused (no smuggled payloads).
/// (Phân phối qua OCI registry (follow-up Spec rev4 — đã chốt): tài liệu
/// policy đi trong config blob OCI để lane ghim theo digest/tag mà không cần
/// protocol registry mới. Layers phải rỗng — còn lại bị từ chối (không payload
/// lén).)
fn validate_policy_artifact(
    media_type: &str,
    layers_are_empty: bool,
) -> PolicyManifestDistributionResult<()> {
    if media_type != super::super::manifest_distribution::POLICY_OCI_CONFIG_MEDIA_TYPE {
        return Err(DistributionError::OciMediaType(media_type.to_string()));
    }
    if !layers_are_empty {
        return Err(DistributionError::OciLayers);
    }
    Ok(())
}

async fn refresh_manifest_from_oci_inner(
    reference: &str,
    trusted_hex_ids: &[String],
    cache_path: &Path,
) -> PolicyManifestDistributionResult<mgc_exec::policy_manifest::PolicyManifest> {
    let (base_url, repo, tag) = super::oci_reference::parse_oci_reference(reference)
        .ok_or_else(|| DistributionError::InvalidOciReference(reference.to_string()))?;
    let mut client = mgc_oci::client::OciClient::new(base_url, None)?;
    if let Some(token) = std::env::var_os(super::super::manifest_distribution::POLICY_OCI_TOKEN_ENV)
        .filter(|value| !value.is_empty())
    {
        client = client.with_token(token.to_string_lossy().into_owned());
    }
    let manifest = client.pull_manifest(&repo, &tag).await?;
    validate_policy_artifact(&manifest.config.media_type, manifest.layers.is_empty())?;
    let bytes = client.pull_blob(&repo, &manifest.config.digest).await?;
    Ok(super::super::manifest_cache::verify_and_cache(
        &bytes,
        trusted_hex_ids,
        cache_path,
    )?)
}

/// Refresh a signed manifest from its OCI config blob.
/// Refresh manifest có ký từ config blob OCI.
pub async fn refresh_manifest_from_oci(
    reference: &str,
    trusted_hex_ids: &[String],
    cache_path: &Path,
) -> anyhow::Result<mgc_exec::policy_manifest::PolicyManifest> {
    refresh_manifest_from_oci_inner(reference, trusted_hex_ids, cache_path)
        .await
        .map_err(anyhow::Error::new)
}

#[cfg(test)]
#[path = "test/oci.rs"]
mod tests;
