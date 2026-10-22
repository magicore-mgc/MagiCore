//! Environment token priority / ưu tiên token environment.

use crate::auth::Auth;

pub(in crate::auth::resolve_auth) fn resolve_environment_token() -> Option<Auth> {
    if let Ok(token) = std::env::var("MGC_NPM_TOKEN")
        && !token.is_empty()
    {
        return Some(Auth {
            token: Some(token),
            ..Default::default()
        });
    }
    if let Ok(token) = std::env::var("NPM_TOKEN")
        && !token.is_empty()
    {
        return Some(Auth {
            token: Some(token),
            ..Default::default()
        });
    }
    None
}
