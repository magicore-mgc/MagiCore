//! Owns trusted token lifetime and active-token capacity constants.
//! Sở hữu constant policy về thời hạn và sức chứa token trusted.

pub const TRUSTED_TOKEN_TTL_SECS: u64 = 15 * 60;

pub const MAX_ACTIVE_TRUSTED_TOKENS: usize = 10_000;
