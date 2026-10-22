//! Shared validation limits — giới hạn xác minh dùng chung.
//!
mod limits;
pub use limits::{
    EXPIRY_LEEWAY_SECS, JWKS_CACHE_TTL_SECS, JWKS_REFRESH_INTERVAL_SECS, MAX_CACHED_ISSUERS,
    MAX_JWT_BYTES, MAX_TOKEN_LIFETIME_SECS,
};
