//! Environment and mounted-file token source — nguồn token từ env và file mount.
//!
mod token_source;
pub(crate) use token_source::token_from_environment;
pub use token_source::{
    OIDC_TOKEN_ENV, OIDC_TOKEN_FILE_ENV, SIGSTORE_OIDC_TOKEN_ENV, SIGSTORE_OIDC_TOKEN_FILE_ENV,
};
