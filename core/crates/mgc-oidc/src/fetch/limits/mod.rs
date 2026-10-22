//! Shared OIDC HTTP limits — giới hạn HTTP OIDC dùng chung.
//!
mod limits;
pub use limits::{MAX_RESPONSE_BYTES, REQUEST_TIMEOUT_SECS};
