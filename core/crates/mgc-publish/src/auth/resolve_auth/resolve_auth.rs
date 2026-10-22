//! Credential source precedence / thứ tự ưu tiên nguồn credentials.

use anyhow::Result;
use mgc_config::npmrc::NpmRc;
use mgc_config::registry::Registry;

use super::{
    cli_token::resolve_cli_token, environment_token::resolve_environment_token,
    error::auth_not_found, npmrc_credentials::resolve_npmrc_credentials,
    registry_credentials::resolve_registry_credentials,
};
use crate::auth::Auth;

/// Resolve credentials in the existing CLI, environment, npmrc, then config order.
/// Phân giải credentials theo thứ tự CLI, environment, npmrc rồi đến config hiện tại.
pub fn resolve_auth(
    npmrc: &NpmRc,
    registry_url: &str,
    config_registry: Option<&Registry>,
    cli_token: Option<&str>,
) -> Result<Auth> {
    if let Some(auth) = resolve_cli_token(cli_token) {
        return Ok(auth);
    }

    if let Some(auth) = resolve_environment_token() {
        return Ok(auth);
    }

    if let Some(auth) = resolve_npmrc_credentials(npmrc, registry_url) {
        return Ok(auth);
    }

    if let Some(auth) = resolve_registry_credentials(registry_url, config_registry)? {
        return Ok(auth);
    }

    Err(auth_not_found(registry_url))
}
