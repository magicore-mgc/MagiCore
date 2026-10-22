//! Credential resolution entry-point test / test entry point phân giải credentials.

use mgc_config::npmrc::NpmRc;

use crate::auth::resolve_auth;

#[test]
fn cli_token_wins_over_npmrc_token() {
    let auth = resolve_auth(
        &NpmRc::parse("//registry.npmjs.org/:_authToken=npmrc").unwrap(),
        "https://registry.npmjs.org/",
        None,
        Some("cli-token"),
    )
    .unwrap();
    assert_eq!(auth.token.as_deref(), Some("cli-token"));
}
