use super::super::{
    trusted_oci_blob_upload::{upload_oci_blob_bytes, upload_oci_blob_file},
    trusted_oci_manifest::trusted_oci_artifact_manifest,
    trusted_oci_repository::{oci_artifact_filename, trusted_oci_registry_root},
    trusted_session::{
        TRUSTED_OCI_REQUEST_MAX_TIMEOUT_SECS, TrustedPublishSession, oci_request_timeout,
    },
};
use anyhow::{Result, bail};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub(in crate::commands) async fn publish_trusted_oci_artifacts(
    registry: &str,
    repository: &str,
    core: &str,
    version: &str,
    artifacts: &[PathBuf],
    session: &mut TrustedPublishSession,
) -> Result<()> {
    use sha2::Digest as _;

    let registry_url = trusted_oci_registry_root(registry)?;
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(TRUSTED_OCI_REQUEST_MAX_TIMEOUT_SECS))
        .build()?;

    for artifact in artifacts {
        if !fs::metadata(artifact)?.is_file() {
            bail!("OCI artifact is not a regular file: {}", artifact.display());
        }
        oci_artifact_filename(artifact)?;
    }

    let (config_digest, config_size) =
        upload_oci_blob_bytes(&client, &registry_url, repository, b"{}", session).await?;
    if config_digest != format!("sha256:{}", hex::encode(sha2::Sha256::digest(b"{}")))
        || config_size != 2
    {
        bail!("OCI empty config digest invariant failed");
    }

    let mut layers = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let filename = oci_artifact_filename(artifact)?.to_owned();
        let (digest, size) =
            upload_oci_blob_file(&client, &registry_url, repository, artifact, session).await?;
        layers.push((filename, digest, size));
    }

    let manifest = trusted_oci_artifact_manifest(core, repository, version, &layers)?;
    let manifest_url = registry_url.join(&format!("/v2/{repository}/manifests/{version}"))?;
    session.refresh_if_needed().await?;
    let response = client
        .put(manifest_url)
        .bearer_auth(session.token())
        .timeout(oci_request_timeout(session.remaining())?)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/vnd.oci.image.manifest.v1+json",
        )
        .body(manifest)
        .send()
        .await?;
    if !response.status().is_success() {
        bail!(
            "OCI artifact manifest upload failed (HTTP {})",
            response.status()
        );
    }
    println!(
        "Published {} OCI artifact layer(s) for {repository}:{version} (core {core}) with trusted provenance.",
        artifacts.len()
    );
    Ok(())
}
