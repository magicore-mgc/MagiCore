//! Registry-scoped npmrc token and Basic credentials / token và Basic theo registry trong npmrc.

use mgc_config::npmrc::NpmRc;

use super::super::registry_candidates::registry_candidates;
use crate::auth::Auth;

pub(in crate::auth::resolve_auth) fn resolve_npmrc_credentials(
    npmrc: &NpmRc,
    registry_url: &str,
) -> Option<Auth> {
    let candidates = registry_candidates(registry_url);
    if let Some(token) = candidates
        .iter()
        .find_map(|candidate| npmrc.token_for(candidate).cloned())
    {
        return Some(Auth {
            token: Some(token),
            ..Default::default()
        });
    }
    if let Some((user, pass)) = candidates
        .iter()
        .find_map(|candidate| npmrc.basic_auth.get(candidate.as_str()).cloned())
    {
        return Some(Auth {
            username: Some(user),
            password: Some(pass),
            ..Default::default()
        });
    }
    None
}
