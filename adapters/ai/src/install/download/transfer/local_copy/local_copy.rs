use mgc_types::{MgError, MgResult};
use std::path::Path;

pub(in crate::install::download) async fn copy_local_to_staging(
    source: &Path,
    staging: &Path,
) -> MgResult<u64> {
    let mut input = tokio::fs::File::open(source)
        .await
        .map_err(|error| MgError::Other(format!("open local model source: {error}")))?;
    let mut output = tokio::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .await
        .map_err(|error| MgError::Other(format!("create model staging file: {error}")))?;
    let copied = tokio::io::copy(&mut input, &mut output)
        .await
        .map_err(|error| MgError::Other(format!("copy local model to staging: {error}")))?;
    output
        .sync_all()
        .await
        .map_err(|error| MgError::Other(format!("sync local model staging file: {error}")))?;
    Ok(copied)
}
