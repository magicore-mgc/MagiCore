use super::super::trusted_session::{TrustedPublishSession, oci_request_timeout};
use super::{
    chunk_upload::{OciRequestAuth, finish_oci_blob_upload, upload_oci_chunk},
    upload_session::begin_oci_blob_upload,
};
use anyhow::{Result, anyhow, bail};
use std::path::Path;

pub(in crate::commands) const OCI_UPLOAD_CHUNK_BYTES: usize = 1024 * 1024;

pub(in crate::commands) async fn upload_oci_blob_bytes(
    client: &reqwest::Client,
    registry: &url::Url,
    repository: &str,
    bytes: &[u8],
    session: &mut TrustedPublishSession,
) -> Result<(String, u64)> {
    use sha2::Digest as _;

    session.refresh_if_needed().await?;
    let mut upload_url = begin_oci_blob_upload(
        client,
        registry,
        repository,
        session.token(),
        oci_request_timeout(session.remaining())?,
    )
    .await?;
    for (index, chunk) in bytes.chunks(OCI_UPLOAD_CHUNK_BYTES).enumerate() {
        let offset = (index * OCI_UPLOAD_CHUNK_BYTES) as u64;
        session.refresh_if_needed().await?;
        upload_oci_chunk(
            client,
            registry,
            repository,
            &mut upload_url,
            offset,
            chunk,
            OciRequestAuth {
                token: session.token(),
                timeout: oci_request_timeout(session.remaining())?,
            },
        )
        .await?;
    }
    let digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(bytes)));
    session.refresh_if_needed().await?;
    finish_oci_blob_upload(
        client,
        &upload_url,
        &digest,
        session.token(),
        oci_request_timeout(session.remaining())?,
    )
    .await?;
    Ok((digest, bytes.len() as u64))
}

pub(in crate::commands) async fn upload_oci_blob_file(
    client: &reqwest::Client,
    registry: &url::Url,
    repository: &str,
    path: &Path,
    session: &mut TrustedPublishSession,
) -> Result<(String, u64)> {
    use sha2::Digest as _;
    use tokio::io::AsyncReadExt as _;

    let mut file = tokio::fs::File::open(path).await?;
    if !file.metadata().await?.is_file() {
        bail!("OCI artifact is not a regular file: {}", path.display());
    }
    session.refresh_if_needed().await?;
    let mut upload_url = begin_oci_blob_upload(
        client,
        registry,
        repository,
        session.token(),
        oci_request_timeout(session.remaining())?,
    )
    .await?;
    let mut hasher = sha2::Sha256::new();
    let mut offset = 0u64;
    let mut buffer = vec![0; OCI_UPLOAD_CHUNK_BYTES];
    loop {
        let count = file.read(&mut buffer).await?;
        if count == 0 {
            break;
        }
        let chunk = &buffer[..count];
        session.refresh_if_needed().await?;
        upload_oci_chunk(
            client,
            registry,
            repository,
            &mut upload_url,
            offset,
            chunk,
            OciRequestAuth {
                token: session.token(),
                timeout: oci_request_timeout(session.remaining())?,
            },
        )
        .await?;
        hasher.update(chunk);
        offset = offset
            .checked_add(count as u64)
            .ok_or_else(|| anyhow!("OCI artifact size overflow"))?;
    }
    let digest = format!("sha256:{}", hex::encode(hasher.finalize()));
    session.refresh_if_needed().await?;
    finish_oci_blob_upload(
        client,
        &upload_url,
        &digest,
        session.token(),
        oci_request_timeout(session.remaining())?,
    )
    .await?;
    Ok((digest, offset))
}
