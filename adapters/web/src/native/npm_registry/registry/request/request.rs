//! registry request.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::{
    NpmRegistry, SCOPED_REGISTRY_TIMEOUT_SECONDS, batch_http_client, credential_path_matches,
    global_http_client, registry_origin_matches,
};
use anyhow::Result;
use std::time::Duration;

impl NpmRegistry {
    pub(in crate::native::npm_registry) fn request_for(
        &self,
        target: &str,
        streaming: bool,
    ) -> Result<reqwest::RequestBuilder> {
        if let Some(clients) = &self.scoped_clients {
            let selected = clients
                .iter()
                .filter(|(_, client)| {
                    client.token.is_some()
                        && client
                            .scope
                            .as_ref()
                            .is_some_and(|scope| credential_path_matches(scope.as_str(), target))
                })
                .max_by_key(|(_, client)| {
                    client.scope.as_ref().map_or(0, |scope| scope.path().len())
                });
            let client = selected
                .or_else(|| clients.first())
                .ok_or_else(|| anyhow::anyhow!("no registry transport configured"))?;
            let request = client.1.get(target)?;
            return Ok(if streaming {
                request.timeout(Duration::from_secs(SCOPED_REGISTRY_TIMEOUT_SECONDS))
            } else {
                request
            });
        }
        let client = if streaming {
            batch_http_client()
        } else {
            global_http_client()
        }
        .map_err(|error| anyhow::anyhow!("{error}"))?;
        Ok(self.with_auth(client.get(target), target))
    }

    pub(in crate::native::npm_registry) fn metadata_request_for(
        &self,
        registry_url: &str,
        token: Option<&str>,
        target: &str,
    ) -> Result<reqwest::RequestBuilder> {
        if let Some(clients) = &self.scoped_clients {
            return clients
                .iter()
                .find(|(url, _)| url == registry_url)
                .ok_or_else(|| anyhow::anyhow!("registry transport is missing"))?
                .1
                .get(target);
        }
        let request = global_http_client()
            .map_err(|error| anyhow::anyhow!("{error}"))?
            .get(target);
        Ok(if let Some(token) = token {
            request.bearer_auth(token)
        } else {
            request
        })
    }

    pub(in crate::native::npm_registry) fn chain(&self) -> Vec<(String, Option<String>)> {
        let mut chain = vec![(self.registry_url.clone(), self.token.clone())];
        chain.extend(self.fallbacks.iter().cloned());
        chain
    }

    pub(in crate::native::npm_registry) fn with_auth(
        &self,
        req: reqwest::RequestBuilder,
        url: &str,
    ) -> reqwest::RequestBuilder {
        if let Some(token) = &self.token
            && registry_origin_matches(&self.registry_url, url)
        {
            return req.header("Authorization", format!("Bearer {token}"));
        }
        req
    }

    pub fn auth_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn registry_url(&self) -> &str {
        &self.registry_url
    }
}
