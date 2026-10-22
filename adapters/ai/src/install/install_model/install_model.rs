//! Coordinate downloading and verifying one AI model artifact.
//! Điều phối tải và xác minh một artifact model AI.

use super::super::download::ModelSource;
use super::super::types::InstallSummary;
use mgc_types::MgResult;
use std::path::Path;

/// Install a model from HuggingFace, a local path, or a URL.
/// Cài model từ HuggingFace, đường dẫn local hoặc URL.
pub async fn install_model(
    model_id: &str,
    source: ModelSource,
    target_dir: &Path,
) -> MgResult<InstallSummary> {
    let start = std::time::Instant::now();

    // Download the model — tải model.
    let (local_path, bytes_downloaded) =
        super::super::download::download_model(model_id, &source, target_dir).await?;

    // Verify a declared checksum — xác minh checksum đã khai báo.
    let verified = if let Some(checksum) = source.checksum() {
        super::super::download::verify_file_checksum(&local_path, checksum)?;
        true
    } else {
        false
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(InstallSummary {
        model_id: model_id.to_string(),
        source,
        local_path,
        verified,
        bytes_downloaded,
        duration_ms,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
