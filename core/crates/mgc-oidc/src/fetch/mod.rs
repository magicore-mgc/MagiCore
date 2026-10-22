//! OIDC token-fetch compatibility facade — facade tương thích cho lấy token OIDC.
//! Public `mgc_oidc::fetch::*` paths stay stable; behavior belongs to named owners.

mod endpoint;
mod fetch;
mod github_actions;
mod limits;
mod token_source;

pub(crate) use endpoint::{fetch_json, secure_url};
pub use fetch::{fetch_sigstore_token, fetch_token};
pub use github_actions::fetch_github_token;
pub use limits::{MAX_RESPONSE_BYTES, REQUEST_TIMEOUT_SECS};
pub use token_source::{
    OIDC_TOKEN_ENV, OIDC_TOKEN_FILE_ENV, SIGSTORE_OIDC_TOKEN_ENV, SIGSTORE_OIDC_TOKEN_FILE_ENV,
};
