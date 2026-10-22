//! OIDC discovery and key retrieval — discovery OIDC và tải khóa.
//!
mod discovery;
pub use discovery::{discover_jwks_url, fetch_jwks};
