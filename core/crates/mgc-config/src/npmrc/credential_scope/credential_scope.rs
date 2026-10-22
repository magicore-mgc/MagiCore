//! Safe URL-scoped npm credential selection / Chọn credential npm theo scope URL an toàn.

use anyhow::Result;

use crate::npmrc::{NpmCredential, NpmRc};

impl NpmRc {
    /// Select URL-scoped credentials; equivalent conflicting scopes fail explicitly.
    /// Chọn token theo host/port/path cụ thể nhất; lỗi không chứa giá trị credential.
    /// Legacy value-only lookup; native transport must use the scoped result.
    /// Giữ API token-only cho caller cũ; đường request mới phải giữ cả scope.
    pub fn token_for_url_checked(&self, registry_url: &str) -> Result<Option<&String>> {
        Ok(self
            .credential_for_url_checked(registry_url)?
            .map(|credential| credential.token))
    }

    pub fn credential_for_url_checked(
        &self,
        registry_url: &str,
    ) -> Result<Option<NpmCredential<'_>>> {
        let target = url::Url::parse(registry_url)
            .map_err(|_| anyhow::anyhow!("invalid registry URL for credential scope matching"))?;
        if !matches!(target.scheme(), "https" | "http")
            || !target.username().is_empty()
            || target.password().is_some()
        {
            return Err(anyhow::anyhow!(
                "invalid registry URL for credential scope matching"
            ));
        }
        // Reject malformed credential scopes instead of silently widening them.
        // Query/fragment trong scope không được biến thành quyền gửi token ở path khác.
        for key in self.auth_tokens.keys() {
            let scoped = url::Url::parse(&format!(
                "{}://{}/",
                target.scheme(),
                key.trim_start_matches('/').trim_end_matches('/')
            ))
            .map_err(|_| anyhow::anyhow!("invalid registry credential scope"))?;
            if scoped.host_str().is_none()
                || !scoped.username().is_empty()
                || scoped.password().is_some()
                || scoped.query().is_some()
                || scoped.fragment().is_some()
            {
                return Err(anyhow::anyhow!("invalid registry credential scope"));
            }
        }
        let candidates: Vec<_> = self
            .auth_tokens
            .iter()
            .filter_map(|(key, token)| {
                let scoped = url::Url::parse(&format!(
                    "{}://{}/",
                    target.scheme(),
                    key.trim_start_matches('/').trim_end_matches('/')
                ))
                .ok()?;
                if !scoped.username().is_empty()
                    || scoped.password().is_some()
                    || scoped.host_str() != target.host_str()
                    || scoped.port_or_known_default() != target.port_or_known_default()
                {
                    return None;
                }
                let prefix = scoped.path();
                let exact = prefix.trim_end_matches('/');
                if target.path() != exact && !target.path().starts_with(prefix) {
                    return None;
                }
                Some((prefix.len(), token, scoped))
            })
            .collect();
        let Some(longest) = candidates.iter().map(|(length, _, _)| *length).max() else {
            return Ok(None);
        };
        let mut matching = candidates
            .iter()
            .filter(|(length, _, _)| *length == longest)
            .map(|(_, token, scope)| (*token, scope));
        let selected = matching
            .next()
            .ok_or_else(|| anyhow::anyhow!("invalid registry credential match state"))?;
        if matching.any(|(token, _)| token != selected.0) {
            return Err(anyhow::anyhow!(
                "conflicting credentials for equivalent registry scopes"
            ));
        }
        Ok(Some(NpmCredential {
            token: selected.0,
            scope: selected.1.clone(),
        }))
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
