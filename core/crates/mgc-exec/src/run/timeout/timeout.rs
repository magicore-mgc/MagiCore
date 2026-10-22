//! Resolve explicit, disabled, or environment-provided execution timeouts.
//! Phân giải timeout tường minh, tắt timeout hoặc cấu hình từ môi trường.

#[path = "error.rs"]
mod error;
use error::TimeoutError;
use std::time::Duration;

/// Default timeout used only when no explicit timeout is supplied.
/// Timeout mặc định chỉ dùng khi caller không truyền timeout riêng.
const DEFAULT_EXEC_TIMEOUT_SECS: u64 = 1800;
const EXEC_TIMEOUT_ENV: &str = "MGC_EXEC_TIMEOUT_SECS";

pub(in crate::run::implementation) fn execution_timeout(
    disabled: bool,
    explicit: Option<Duration>,
) -> std::result::Result<Option<Duration>, TimeoutError> {
    execution_timeout_with(disabled, explicit, || std::env::var_os(EXEC_TIMEOUT_ENV))
}

fn execution_timeout_with(
    disabled: bool,
    explicit: Option<Duration>,
    configured_value: impl FnOnce() -> Option<std::ffi::OsString>,
) -> std::result::Result<Option<Duration>, TimeoutError> {
    if disabled {
        return Ok(None);
    }
    if let Some(timeout) = explicit {
        return Ok(Some(timeout));
    }
    resolve_configured_timeout(configured_value()).map(Some)
}

fn resolve_configured_timeout(
    value: Option<std::ffi::OsString>,
) -> std::result::Result<Duration, TimeoutError> {
    let Some(value) = value else {
        return Ok(Duration::from_secs(DEFAULT_EXEC_TIMEOUT_SECS));
    };
    let value = value.to_str().ok_or(TimeoutError::NonUnicodeValue)?;
    parse_timeout_value(value)
}

fn parse_timeout_value(value: &str) -> std::result::Result<Duration, TimeoutError> {
    let seconds = value
        .parse::<u64>()
        .map_err(TimeoutError::InvalidUnsignedSeconds)?;
    if seconds == 0 {
        return Err(TimeoutError::ZeroSeconds);
    }
    Ok(Duration::from_secs(seconds))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
