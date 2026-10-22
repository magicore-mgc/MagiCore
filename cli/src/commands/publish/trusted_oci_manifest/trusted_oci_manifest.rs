use super::super::trusted_oci_image::valid_oci_tag;
use anyhow::{Result, anyhow, bail};

/// Build an OCI 1.1 artifact manifest whose descriptors bind the files to one core.
/// Tạo manifest OCI 1.1 gắn descriptor của các file với đúng một core.
pub(in crate::commands) fn trusted_oci_artifact_manifest(
    core: &str,
    repository: &str,
    version: &str,
    layers: &[(String, String, u64)],
) -> Result<Vec<u8>> {
    use sha2::Digest as _;

    let core_id = mgc_types::Ecosystem::from_str(core)
        .ok_or_else(|| anyhow!("unknown MagiCore core: {core}"))?
        .as_str();
    let prefix = format!("{core_id}/");
    let package = repository
        .strip_prefix(&prefix)
        .filter(|package| !package.is_empty())
        .ok_or_else(|| anyhow!("OCI repository does not match the selected core"))?;
    let expected_repository =
        mgc_registry_server::trusted::core_scoped_oci_repository(core_id, package)
            .map_err(|_| anyhow!("invalid core-scoped OCI repository"))?;
    if expected_repository != repository || !valid_oci_tag(version) || layers.is_empty() {
        bail!("invalid OCI artifact manifest metadata");
    }

    let layer_descriptors = layers
        .iter()
        .map(|(filename, digest, size)| {
            let Some(hash) = digest.strip_prefix("sha256:") else {
                bail!("OCI artifact layer has an invalid SHA-256 digest");
            };
            if hash.len() != 64
                || !hash
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                || filename.is_empty()
                || filename.len() > 255
                || filename
                    .bytes()
                    .any(|byte| byte.is_ascii_control() || matches!(byte, b'/' | b'\\'))
            {
                bail!("OCI artifact layer metadata is invalid");
            }
            Ok(serde_json::json!({
                "mediaType": "application/vnd.magicore.artifact.layer.v1",
                "digest": digest,
                "size": size,
                "annotations": {
                    "org.opencontainers.image.title": filename,
                },
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    let config_digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(b"{}")));
    let manifest = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.manifest.v1+json",
        "artifactType": "application/vnd.magicore.artifact.v1",
        "config": {
            "mediaType": "application/vnd.oci.empty.v1+json",
            "digest": config_digest,
            "size": 2,
        },
        "layers": layer_descriptors,
        "annotations": {
            "io.magicore.core": core_id,
            "io.magicore.package": repository,
            "io.magicore.version": version,
        },
    });
    Ok(serde_json::to_vec(&manifest)?)
}
