use mgc_types::{MgError, MgResult};

const DEFAULT_MAX_REMOTE_MODEL_BYTES: u64 = 64 * 1024 * 1024 * 1024;
pub(in crate::install) const MAX_REMOTE_MODEL_BYTES_ENV: &str = "MGC_MAX_MODEL_DOWNLOAD_BYTES";

pub(in crate::install) fn max_remote_model_bytes() -> MgResult<u64> {
    let configured = std::env::var(MAX_REMOTE_MODEL_BYTES_ENV).ok();
    let max_bytes = match configured {
        Some(value) => value.parse::<u64>().map_err(|error| {
            MgError::Integrity(format!(
                "{MAX_REMOTE_MODEL_BYTES_ENV} must be a positive byte count: {error}"
            ))
        })?,
        None => DEFAULT_MAX_REMOTE_MODEL_BYTES,
    };
    if max_bytes == 0 {
        return Err(MgError::Integrity(format!(
            "{MAX_REMOTE_MODEL_BYTES_ENV} must be greater than zero"
        )));
    }
    Ok(max_bytes)
}

pub(in crate::install) fn next_model_download_total(
    current: u64,
    chunk_len: usize,
    max_bytes: u64,
) -> MgResult<u64> {
    let next = current
        .checked_add(chunk_len as u64)
        .ok_or_else(|| MgError::Integrity("model download size overflow".into()))?;
    if next > max_bytes {
        return Err(MgError::Integrity(format!(
            "model download exceeds configured size limit ({max_bytes} bytes)"
        )));
    }
    Ok(next)
}
