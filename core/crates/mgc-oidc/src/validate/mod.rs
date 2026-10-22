//! OIDC JWT validation facade — facade xác minh JWT OIDC.
//! Giữ nguyên public route `mgc_oidc::validate::*`; các behavior nằm ở owner riêng.

mod discovery;
mod issuer_claim;
mod jwks;
mod jwks_cache;
mod limits;
mod validate;
mod validate_with_discovery;

pub use discovery::{discover_jwks_url, fetch_jwks};
pub(crate) use issuer_claim::unverified_issuer;
pub use jwks::{Jwk, Jwks};
pub(crate) use jwks_cache::cached_jwks;
pub use limits::{
    EXPIRY_LEEWAY_SECS, JWKS_CACHE_TTL_SECS, JWKS_REFRESH_INTERVAL_SECS, MAX_CACHED_ISSUERS,
    MAX_JWT_BYTES, MAX_TOKEN_LIFETIME_SECS,
};
pub use validate::validate;
pub use validate_with_discovery::validate_with_discovery;
