//! Serialize refreshes and retain fetched issuer keys — tuần tự refresh và giữ khóa issuer.
//!
use crate::validate::fetch_jwks;
#[path = "types.rs"]
mod types;
use self::types::CachedKeys;
use crate::error::OidcError;
use crate::validate::discovery::discover_jwks_url;
use crate::validate::jwks::Jwks;
use crate::validate::{JWKS_CACHE_TTL_SECS, JWKS_REFRESH_INTERVAL_SECS, MAX_CACHED_ISSUERS};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Refresh is serialized and throttled, including failed requests.
/// Gộp các request refresh đồng thời và giới hạn cả lần fetch thất bại.
pub(crate) async fn cached_jwks(issuer: &str, kid: &str) -> Result<Jwks, OidcError> {
    type IssuerCache = Mutex<HashMap<String, Arc<Mutex<Option<CachedKeys>>>>>;
    static CACHE: OnceLock<IssuerCache> = OnceLock::new();
    let slot = {
        let mut cache = CACHE
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .await;
        if !cache.contains_key(issuer) && cache.len() >= MAX_CACHED_ISSUERS {
            return Err(OidcError::VerificationFailed(
                "trusted issuer cache limit reached".into(),
            ));
        }
        cache
            .entry(issuer.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(None)))
            .clone()
    };
    // Different issuers never share a network lock — issuer khác không chờ chung khóa mạng.
    let mut slot = slot.lock().await;
    if let Some(entry) = slot.as_ref() {
        let fresh = entry.fetched_at.elapsed() < Duration::from_secs(JWKS_CACHE_TTL_SECS);
        if fresh
            && let Some(keys) = &entry.keys
            && keys.keys.iter().any(|key| key.kid.as_deref() == Some(kid))
        {
            return Ok(keys.clone());
        }
        if entry.attempted_at.elapsed() < Duration::from_secs(JWKS_REFRESH_INTERVAL_SECS) {
            return Err(OidcError::VerificationFailed(
                "JWKS refresh temporarily throttled".into(),
            ));
        }
    }
    let now = Instant::now();
    let entry = slot.get_or_insert(CachedKeys {
        keys: None,
        fetched_at: now,
        attempted_at: now,
    });
    entry.attempted_at = now;
    let url = discover_jwks_url(issuer).await?;
    let keys = fetch_jwks(&url).await?;
    entry.keys = Some(keys.clone());
    entry.fetched_at = Instant::now();
    Ok(keys)
}
