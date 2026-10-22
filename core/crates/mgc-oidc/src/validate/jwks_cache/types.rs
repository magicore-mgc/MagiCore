//! Private cache entry state — trạng thái entry cache nội bộ.
//!
use crate::validate::Jwks;
use std::time::Instant;

pub(super) struct CachedKeys {
    pub(super) keys: Option<Jwks>,
    pub(super) fetched_at: Instant,
    pub(super) attempted_at: Instant,
}
