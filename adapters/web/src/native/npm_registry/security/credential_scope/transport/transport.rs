//! security credential scope transport.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::{
    SCOPED_REGISTRY_TIMEOUT_SECONDS, ScopedRegistryClient, credential_path_matches,
    global_http_client,
};
use anyhow::Result;
use std::time::Duration;

impl ScopedRegistryClient {
    /// Build clients once; missing or malformed credential scope fails explicitly.
    /// Credential có token thì phải có scope hợp lệ trước khi tạo transport.
    pub fn new(registry: &mgc_config::chain::ScopedRegistry) -> Result<Self> {
        let scope = registry.credential_scope.clone();
        let token = registry.registry.token.clone();
        let credential_client = if token.is_some() {
            let selected = scope
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("credential scope is missing"))?;
            if !credential_path_matches(selected.as_str(), selected.as_str()) {
                return Err(anyhow::anyhow!("invalid registry credential scope"));
            }
            let selected = selected.to_string();
            Some(
                reqwest::Client::builder()
                    .redirect(reqwest::redirect::Policy::custom(move |attempt| {
                        if !credential_path_matches(&selected, attempt.url().as_str()) {
                            return attempt.error(std::io::Error::other(
                                "credential redirect leaves authorized registry scope",
                            ));
                        }
                        reqwest::redirect::Policy::default().redirect(attempt)
                    }))
                    .timeout(Duration::from_secs(SCOPED_REGISTRY_TIMEOUT_SECONDS))
                    .user_agent(format!("MagiCore/{}", env!("CARGO_PKG_VERSION")))
                    .build()?,
            )
        } else {
            None
        };
        Ok(Self {
            scope,
            token,
            credential_client,
            public_client: global_http_client().map_err(|error| anyhow::anyhow!("{error}"))?,
        })
    }

    /// Reuse the appropriate client; public requests never inherit credential headers.
    /// Mỗi request kiểm scope lại, không đặt token vào default headers của client.
    pub fn get(&self, target: &str) -> Result<reqwest::RequestBuilder> {
        let parsed = reqwest::Url::parse(target)
            .map_err(|_| anyhow::anyhow!("invalid registry request URL"))?;
        if !matches!(parsed.scheme(), "http" | "https")
            || parsed.host_str().is_none()
            || !parsed.username().is_empty()
            || parsed.password().is_some()
        {
            return Err(anyhow::anyhow!(
                "registry request URL must use HTTP without userinfo"
            ));
        }
        let authorized = self.token.is_some()
            && self
                .scope
                .as_ref()
                .is_some_and(|scope| credential_path_matches(scope.as_str(), target));
        if authorized {
            let client = self
                .credential_client
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("credential transport is missing"))?;
            let token = self
                .token
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("credential token is missing"))?;
            Ok(client.get(parsed).bearer_auth(token))
        } else {
            Ok(self.public_client.get(parsed))
        }
    }
}
