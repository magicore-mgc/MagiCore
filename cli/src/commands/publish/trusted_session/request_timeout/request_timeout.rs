//! Trusted-publish request timeouts bounded by token expiry.
//! Timeout request trusted-publish bị chặn trên bởi hạn token.

use std::time::Duration;

use anyhow::{Result, anyhow};

use super::super::TRUSTED_TOKEN_EXPIRY_SKEW_SECS;

pub(in crate::commands) const TRUSTED_DOCKER_MAX_PUSH_SECS: u64 = 1800;
pub(in crate::commands) const TRUSTED_NPM_REQUEST_TIMEOUT_SECS: u64 = 600;
pub(in crate::commands) const TRUSTED_OCI_REQUEST_MAX_TIMEOUT_SECS: u64 = 1800;

pub(in crate::commands) fn docker_push_timeout(remaining: Duration) -> Result<Duration> {
    let usable = remaining
        .checked_sub(Duration::from_secs(TRUSTED_TOKEN_EXPIRY_SKEW_SECS))
        .filter(|duration| !duration.is_zero())
        .ok_or_else(|| anyhow!("trusted publish token expires too soon to start a Docker push"))?;
    Ok(usable.min(Duration::from_secs(TRUSTED_DOCKER_MAX_PUSH_SECS)))
}

pub(in crate::commands) fn oci_request_timeout(remaining: Duration) -> Result<Duration> {
    let usable = remaining
        .checked_sub(Duration::from_secs(TRUSTED_TOKEN_EXPIRY_SKEW_SECS))
        .filter(|duration| !duration.is_zero())
        .ok_or_else(|| anyhow!("trusted publish token expires too soon for an OCI request"))?;
    Ok(usable.min(Duration::from_secs(TRUSTED_OCI_REQUEST_MAX_TIMEOUT_SECS)))
}

pub(in crate::commands) fn trusted_npm_request_timeout(remaining: Duration) -> Result<Duration> {
    let usable = remaining
        .checked_sub(Duration::from_secs(TRUSTED_TOKEN_EXPIRY_SKEW_SECS))
        .filter(|duration| !duration.is_zero())
        .ok_or_else(|| anyhow!("trusted publish token expires too soon for an npm request"))?;
    Ok(usable.min(Duration::from_secs(TRUSTED_NPM_REQUEST_TIMEOUT_SECS)))
}
