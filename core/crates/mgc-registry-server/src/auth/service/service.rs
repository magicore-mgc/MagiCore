//! Authentication service construction.
// Khởi tạo AuthService và sở hữu các operation service trong module con.

use crate::auth::AuthService;
use crate::storage::RegistryStore;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

impl AuthService {
    pub fn new(admin_token: Option<String>, store: Arc<RegistryStore>) -> Self {
        Self {
            admin_token: admin_token.map(|token| {
                crate::credential_security::TokenDigest::from_secret(&token).into_string()
            }),
            users: Mutex::new(HashMap::new()),
            scopes: HashMap::new(),
            store,
            trusted_config: None,
            trusted_tokens: Mutex::new(HashMap::new()),
            attestation_key: None,
            allow_insecure_trusted_http: false,
            trusted_proxy_ips: Vec::new(),
        }
    }
}
