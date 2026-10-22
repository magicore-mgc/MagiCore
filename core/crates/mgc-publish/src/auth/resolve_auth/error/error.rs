//! Existing resolver error construction / tạo error hiện có của resolver.

use anyhow::{Error, anyhow};

pub(in crate::auth::resolve_auth) fn auth_not_found(registry_url: &str) -> Error {
    anyhow!(
        "Auth not found for registry {} — set MGC_NPM_TOKEN or NPM_TOKEN, .npmrc, or mgc.toml [registry]",
        registry_url
    )
}

pub(in crate::auth::resolve_auth) fn auth_type_token_missing(registry_url: &str) -> Error {
    anyhow!(
        "registry {} declares auth_type = \"token\" but mgc.toml [registry] has no token",
        registry_url
    )
}

pub(in crate::auth::resolve_auth) fn auth_type_basic_missing(registry_url: &str) -> Error {
    anyhow!(
        "registry {} declares auth_type = \"basic\" but mgc.toml has no [registry] username/password",
        registry_url
    )
}
