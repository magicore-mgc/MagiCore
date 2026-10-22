//! Authentication service clone semantics.
// Clone state/cache theo semantics hiện có.

use crate::auth::AuthService;
use std::sync::Mutex;

impl Clone for AuthService {
    fn clone(&self) -> Self {
        Self {
            admin_token: self.admin_token.clone(),
            users: Mutex::new(self.users_guard().clone()),
            scopes: self.scopes.clone(),
            store: self.store.clone(),
            trusted_config: self.trusted_config.clone(),
            trusted_tokens: Mutex::new(self.trusted_tokens_guard().clone()),
            attestation_key: self.attestation_key.clone(),
            allow_insecure_trusted_http: self.allow_insecure_trusted_http,
            trusted_proxy_ips: self.trusted_proxy_ips.clone(),
        }
    }
}
