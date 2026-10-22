//! Tsconfig owner wiring — nối owner tsconfig.
#[path = "error.rs"]
mod error;
#[path = "tsconfig.rs"]
mod implementation;
pub(super) use self::error::TsconfigDigestError;
pub(super) use self::implementation::tsconfig_digest;
