//! npmrc credential source tests / test nguồn credentials npmrc.

use mgc_config::npmrc::NpmRc;

use super::super::resolve_npmrc_credentials;

#[test]
fn scoped_npmrc_token_is_selected() {
    let npmrc = NpmRc::parse("//registry.example/:_authToken=token").unwrap();
    let auth = resolve_npmrc_credentials(&npmrc, "https://registry.example/").unwrap();
    assert_eq!(auth.token.as_deref(), Some("token"));
}

#[test]
fn scoped_npmrc_basic_auth_is_selected() {
    let npmrc = NpmRc::parse(
        "//registry.example/:username=user
//registry.example/:_password=pass",
    )
    .unwrap();
    let auth = resolve_npmrc_credentials(&npmrc, "https://registry.example/npm").unwrap();
    assert_eq!(auth.username.as_deref(), Some("user"));
    assert_eq!(auth.password.as_deref(), Some("pass"));
}
