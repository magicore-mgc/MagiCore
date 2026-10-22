//! Authentication service state.
use crate::auth::User;
// State AuthService; field nội bộ chỉ mở trong module auth.

use crate::auth::TrustedToken;
use crate::storage::RegistryStore;
use crate::trusted::TrustedPublishingConfig;
use mgc_crypto::KeyPair;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

/// Authentication service — users persist trong SQLite, cache Mutex cho đọc nhanh
pub struct AuthService {
    pub admin_token: Option<String>,
    pub(in crate::auth) users: Mutex<HashMap<String, User>>, // token -> user (cache; source of truth = DB)
    pub scopes: HashMap<String, Vec<String>>,                // scope -> allowed packages
    pub(in crate::auth) store: Arc<RegistryStore>,
    pub(in crate::auth) trusted_config: Option<TrustedPublishingConfig>,
    pub(in crate::auth) trusted_tokens: Mutex<HashMap<String, TrustedToken>>,
    pub(in crate::auth) attestation_key: Option<KeyPair>,
    pub(in crate::auth) allow_insecure_trusted_http: bool,
    pub(in crate::auth) trusted_proxy_ips: Vec<IpAddr>,
}
