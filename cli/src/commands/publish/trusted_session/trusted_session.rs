use super::token_exchange::trusted_publish_session;
use super::types::TrustedPublishSession;
use anyhow::{Result, anyhow, bail};
use std::time::Duration;

pub(in crate::commands) const TRUSTED_TOKEN_REFRESH_MARGIN_SECS: u64 = 60;

pub(in crate::commands) const TRUSTED_TOKEN_EXPIRY_SKEW_SECS: u64 = 5;

pub(in crate::commands) const TRUSTED_PYPI_REQUEST_TIMEOUT_SECS: u64 = 300;

impl TrustedPublishSession {
    pub(in crate::commands::publish) fn token(&self) -> &str {
        &self.token
    }

    pub(in crate::commands::publish) fn remaining(&self) -> Duration {
        self.expires_at
            .saturating_duration_since(tokio::time::Instant::now())
    }

    pub(in crate::commands::publish) async fn refresh_if_needed(&mut self) -> Result<()> {
        self.refresh_for(Duration::from_secs(TRUSTED_TOKEN_REFRESH_MARGIN_SECS))
            .await
    }

    pub(in crate::commands::publish) async fn refresh_for(
        &mut self,
        minimum_lifetime: Duration,
    ) -> Result<()> {
        let now = tokio::time::Instant::now();
        if !trusted_token_expires_within(now, self.expires_at, minimum_lifetime) {
            return Ok(());
        }
        let refreshed = trusted_publish_session(
            &self.registry,
            &self.package,
            &self.protocol,
            self.audience.as_deref(),
        )
        .await
        .map_err(|error| {
            anyhow!("could not refresh the trusted token before expiry; verify that this CI environment can mint a fresh OIDC token: {error}")
        })?;
        if trusted_token_expires_within(
            tokio::time::Instant::now(),
            refreshed.expires_at,
            minimum_lifetime,
        ) {
            bail!(
                "refreshed trusted token lifetime is shorter than the next upload request timeout"
            );
        }
        *self = refreshed;
        Ok(())
    }
}

#[path = "request_timeout/mod.rs"]
mod request_timeout;

pub(in crate::commands) use request_timeout::{
    TRUSTED_NPM_REQUEST_TIMEOUT_SECS, TRUSTED_OCI_REQUEST_MAX_TIMEOUT_SECS, docker_push_timeout,
    oci_request_timeout, trusted_npm_request_timeout,
};

pub(in crate::commands) fn trusted_token_expires_within(
    now: tokio::time::Instant,
    expires_at: tokio::time::Instant,
    minimum_lifetime: Duration,
) -> bool {
    expires_at <= now + minimum_lifetime
}

pub(in crate::commands) fn trusted_token_deadline(
    exchange_started: tokio::time::Instant,
    expires_in: Duration,
) -> tokio::time::Instant {
    exchange_started + expires_in
}

/// Refreshes trusted npm credentials and bounds one registry request by token expiry.
/// Làm mới token npm trusted và giới hạn thời gian request theo hạn token.
pub(in crate::commands) async fn apply_npm_request_auth(
    request: reqwest::RequestBuilder,
    auth: &mgc_publish::auth::Auth,
    trusted_session: Option<&mut TrustedPublishSession>,
) -> Result<reqwest::RequestBuilder> {
    if let Some(session) = trusted_session {
        session
            .refresh_for(Duration::from_secs(
                TRUSTED_NPM_REQUEST_TIMEOUT_SECS + TRUSTED_TOKEN_EXPIRY_SKEW_SECS,
            ))
            .await?;
        let timeout = trusted_npm_request_timeout(session.remaining())?;
        Ok(request.timeout(timeout).bearer_auth(session.token()))
    } else if let Some(header) = auth.header_value() {
        Ok(request.header("Authorization", header))
    } else {
        Ok(request)
    }
}
