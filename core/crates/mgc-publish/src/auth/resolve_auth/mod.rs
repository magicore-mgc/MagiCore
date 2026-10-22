//! Credential resolution owners / các owner phân giải credentials.

mod cli_token;
mod environment_token;
mod error;
mod npmrc_credentials;
mod registry_candidates;
mod registry_credentials;
mod resolve_auth;
#[cfg(test)]
mod test;

pub use resolve_auth::resolve_auth;
