//! Archive size policy and environment configuration.
//! Chính sách giới hạn kích thước archive và cấu hình môi trường.
use mgc_types::{MgError, MgResult};

pub(in crate::install::pub_archive_cache) const ARCHIVE_DIRECTORY: &str = "archives";
pub(in crate::install::pub_archive_cache) const DEFAULT_ARCHIVE_MAX_BYTES: u64 = 256 * 1024 * 1024;
pub(in crate::install::pub_archive_cache) const ARCHIVE_MAX_BYTES_ENV: &str =
    "MGC_PUB_ARCHIVE_MAX_BYTES";

pub(in crate::install::pub_archive_cache) fn archive_limit() -> MgResult<u64> {
    match std::env::var(ARCHIVE_MAX_BYTES_ENV) {
        Ok(value) => parse_archive_limit(&value),
        Err(std::env::VarError::NotPresent) => Ok(DEFAULT_ARCHIVE_MAX_BYTES),
        Err(error) => Err(MgError::Other(format!(
            "read {ARCHIVE_MAX_BYTES_ENV}: {error}"
        ))),
    }
}

fn parse_archive_limit(value: &str) -> MgResult<u64> {
    value
        .parse::<u64>()
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| {
            MgError::Other(format!(
                "{ARCHIVE_MAX_BYTES_ENV} must be a positive integer"
            ))
        })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
