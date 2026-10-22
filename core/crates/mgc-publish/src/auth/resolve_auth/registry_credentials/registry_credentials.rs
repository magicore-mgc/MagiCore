//! Configured registry authentication / xác thực registry trong config.

use anyhow::Result;
use mgc_config::registry::Registry;

use super::super::error::{auth_type_basic_missing, auth_type_token_missing};
use crate::auth::Auth;

pub(in crate::auth::resolve_auth) fn resolve_registry_credentials(
    registry_url: &str,
    registry: Option<&Registry>,
) -> Result<Option<Auth>> {
    let Some(registry) = registry else {
        return Ok(None);
    };

    let force_basic = registry.auth_type.as_deref() == Some("basic");
    let force_token = registry.auth_type.as_deref() == Some("token");
    if !force_basic && let Some(token) = &registry.token {
        return Ok(Some(Auth {
            token: Some(token.clone()),
            ..Default::default()
        }));
    }
    if !force_token && let (Some(user), Some(pass)) = (&registry.username, &registry.password) {
        return Ok(Some(Auth {
            username: Some(user.clone()),
            password: Some(pass.clone()),
            ..Default::default()
        }));
    }
    if force_token && registry.token.is_none() {
        return Err(auth_type_token_missing(registry_url));
    }
    if force_basic && (registry.username.is_none() || registry.password.is_none()) {
        return Err(auth_type_basic_missing(registry_url));
    }

    Ok(None)
}
