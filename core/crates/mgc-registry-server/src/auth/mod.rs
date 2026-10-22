// Authentication owner modules. / Modules propriétaires de l’authentification.
pub(in crate::auth) mod auth;
pub(in crate::auth) mod middleware;
pub(in crate::auth) mod policy;
pub(in crate::auth) mod request;
pub(in crate::auth) mod service;
pub(in crate::auth) mod types;
#[cfg(test)]
use crate::storage::RegistryStore;
pub use auth::*;
#[cfg(test)]
use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, StatusCode},
};
#[cfg(test)]
use base64::Engine;
#[cfg(test)]
use mgc_oidc::OidcClaims;
pub(in crate::auth) use request::authentication::authentication::trusted_token_secret;
#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use std::time::{Duration, Instant};
pub(in crate::auth) use types::trusted_token::trusted_token::TrustedToken;
#[cfg(test)]
#[path = "../test/auth.rs"]
mod tests;
