// huggingface owner — huggingface owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::cas_pull::cas_import;
use super::super::manifest_path::validate_model_name;
use super::HuggingFaceError;
use anyhow::{Context, Result};
/// Pull a revision-pinned Hugging Face artifact and verify its SHA-256 before CAS import.
/// Tải artifact Hugging Face đã ghim revision và xác minh SHA-256 trước khi nhập CAS.
pub(in crate::commands::model) async fn pull_hf(
    store: &mgc_store::cas::ContentStore,
    hf: &str,
    revision: Option<&str>,
    sha256: Option<&str>,
    max_bytes: u64,
) -> Result<(String, Vec<String>, u64, String)> {
    let (name, url, revision, expected_sha256) = parse_hf_locator(hf, revision, sha256)?;
    let response = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(20))
        .timeout(std::time::Duration::from_secs(2 * 60 * 60))
        .build()?
        .get(url.as_str())
        .send()
        .await
        .with_context(|| HuggingFaceError::RequestFailed)?;
    if !response.status().is_success() {
        return Err(HuggingFaceError::DownloadFailed {
            status: response.status().to_string(),
            url: url.to_string(),
        }
        .into());
    }
    let declared_length = response.content_length();
    if declared_length.is_some_and(|declared| declared > max_bytes) {
        return Err(HuggingFaceError::ArtifactExceedsLimit(max_bytes).into());
    }
    let tmp = std::env::temp_dir().join(format!(
        "mgc-hf-{}-{}.part",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let result = async {
        use sha2::Digest;
        use tokio::io::AsyncWriteExt;

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
            .await?;
        let mut response = response;
        let mut hasher = sha2::Sha256::new();
        let mut total = 0_u64;
        while let Some(chunk) = response.chunk().await? {
            total = next_hf_download_total(total, chunk.len(), max_bytes)?;
            hasher.update(&chunk);
            file.write_all(&chunk).await?;
        }
        if declared_length.is_some_and(|declared| declared != total) {
            return Err(HuggingFaceError::ContentLengthMismatch {
                expected: declared_length.unwrap_or_default(),
                actual: total,
            }
            .into());
        }
        file.sync_all().await?;
        drop(file);
        let actual = hex::encode(hasher.finalize());
        if !actual.eq_ignore_ascii_case(&expected_sha256) {
            return Err(HuggingFaceError::Sha256Mismatch {
                expected: expected_sha256,
                actual,
            }
            .into());
        }
        let (hash, imported_len) = cas_import(store, &tmp)?;
        if imported_len != total {
            return Err(HuggingFaceError::ArtifactChangedDuringImport.into());
        }
        let pinned_source = format!("hf://{hf}@{revision}#sha256={expected_sha256}");
        Ok((name, vec![hash], total, pinned_source))
    }
    .await;
    let _ = tokio::fs::remove_file(&tmp).await;
    result
}

pub(in crate::commands::model) fn next_hf_download_total(
    current: u64,
    chunk_len: usize,
    max_bytes: u64,
) -> Result<u64> {
    let next = current
        .checked_add(chunk_len as u64)
        .ok_or(HuggingFaceError::SizeOverflow)?;
    if next > max_bytes {
        return Err(HuggingFaceError::DownloadLimitExceeded(max_bytes).into());
    }
    Ok(next)
}

pub(in crate::commands::model) fn parse_hf_locator(
    hf: &str,
    revision: Option<&str>,
    sha256: Option<&str>,
) -> Result<(String, url::Url, String, String)> {
    let revision = revision.ok_or(HuggingFaceError::RevisionRequired)?;
    if revision.len() != 40 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(HuggingFaceError::InvalidRevision.into());
    }
    let sha256 = sha256.ok_or(HuggingFaceError::Sha256Required)?;
    if sha256.len() != 64 || !sha256.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(HuggingFaceError::InvalidSha256.into());
    }
    let segments = hf.split('/').collect::<Vec<_>>();
    if segments.len() < 3
        || segments.iter().any(|part| {
            part.is_empty() || matches!(*part, "." | "..") || part.contains(['\\', '\0', ':'])
        })
    {
        return Err(HuggingFaceError::InvalidSource.into());
    }
    let name = segments.join("/");
    validate_model_name(&name)?;
    let mut url = url::Url::parse("https://huggingface.co/")?;
    {
        let mut path = url
            .path_segments_mut()
            .map_err(|_| HuggingFaceError::UnableToConstructUrl)?;
        path.pop_if_empty()
            .push(segments[0])
            .push(segments[1])
            .push("resolve")
            .push(revision);
        for segment in &segments[2..] {
            path.push(segment);
        }
    }
    Ok((name, url, revision.to_string(), sha256.to_ascii_lowercase()))
}
