use super::super::ModelSource;
use super::super::destination::{ensure_real_directory, ensure_safe_model_destination};
use super::super::filename_from_url;
use super::super::staging::unique_staging_path;
use super::super::transfer::{copy_local_to_staging, http_download_to_staging};
use super::super::validation::{validate_model_relative_path, validate_remote_integrity};
use super::super::verify_file_checksum;
use super::types::{PreparedModelDownload, PreparedTransferSource};
use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

/// Download to same-directory staging, verify a declared checksum, then atomically publish.
/// Download vào staging cùng thư mục, verify checksum nếu có, rồi publish nguyên tử.
pub async fn download_model(
    _model_id: &str,
    source: &ModelSource,
    target_dir: &Path,
) -> MgResult<(PathBuf, u64)> {
    validate_remote_integrity(source)?;

    let prepared = match source {
        ModelSource::Local(src) => {
            let filename = src
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| MgError::Other("local model source has no valid filename".into()))?;
            PreparedModelDownload {
                relative_path: validate_model_relative_path(filename)?,
                transfer_source: PreparedTransferSource::Local(src),
            }
        }
        ModelSource::HuggingFace {
            repo_id,
            revision,
            filename,
            ..
        } => {
            let relative = validate_model_relative_path(filename)?;
            let repo = validate_model_relative_path(repo_id)?;
            let rev = validate_model_relative_path(revision.as_deref().ok_or_else(|| {
                MgError::Integrity(
                    "Hugging Face model source requires an immutable commit revision".into(),
                )
            })?)?;
            let url = format!(
                "https://huggingface.co/{}/resolve/{}/{}",
                repo.display(),
                rev.display(),
                relative.display()
            );
            PreparedModelDownload {
                relative_path: relative,
                transfer_source: PreparedTransferSource::Remote(url),
            }
        }
        ModelSource::Url { url, .. } => {
            let filename = filename_from_url(url);
            PreparedModelDownload {
                relative_path: validate_model_relative_path(&filename)?,
                transfer_source: PreparedTransferSource::Remote(url.clone()),
            }
        }
    };

    let PreparedModelDownload {
        relative_path,
        transfer_source,
    } = prepared;
    ensure_real_directory(target_dir)?;
    let dest = ensure_safe_model_destination(target_dir, &relative_path)?;
    let staging = unique_staging_path(&dest);
    let transfer = async {
        let bytes = match transfer_source {
            PreparedTransferSource::Local(source) => {
                copy_local_to_staging(source, &staging).await?
            }
            PreparedTransferSource::Remote(url) => http_download_to_staging(&url, &staging).await?,
        };

        if let Some(expected) = source.checksum() {
            verify_file_checksum(&staging, expected)?;
        }

        mgc_lockfile::atomic::atomic_replace_file(&staging, &dest).map_err(|error| {
            MgError::Other(format!("publish model artifact atomically: {error}"))
        })?;
        #[cfg(unix)]
        std::fs::File::open(dest.parent().unwrap_or(target_dir))
            .and_then(|directory| directory.sync_all())
            .map_err(|error| MgError::Other(format!("sync model artifact directory: {error}")))?;
        Ok((dest.clone(), bytes))
    }
    .await;
    if transfer.is_err() {
        let _ = tokio::fs::remove_file(&staging).await;
    }
    transfer
}
