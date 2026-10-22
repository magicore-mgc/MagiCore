#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn selects_longest_matching_path_and_keeps_its_scope() {
    let npmrc = NpmRc::parse(
        "//registry.example/:_authToken=root-token\n//registry.example/team/:_authToken=team-token\n",
    )
    .unwrap();
    let credential = npmrc
        .credential_for_url_checked("https://registry.example/team/pkg")
        .unwrap()
        .unwrap();
    assert_eq!(credential.token, "team-token");
    assert_eq!(credential.scope.path(), "/team/");
    assert!(
        npmrc
            .credential_for_url_checked("https://registry.example/other/pkg")
            .unwrap()
            .unwrap()
            .token
            .eq("root-token")
    );
}

#[test]
fn rejects_malformed_scopes_and_urls_with_user_info() {
    let malformed = NpmRc::parse("//registry.example/?team:_authToken=fixture\n").unwrap();
    assert!(
        malformed
            .credential_for_url_checked("https://registry.example/other/")
            .is_err()
    );

    let npmrc = NpmRc::parse("//registry.example/:_authToken=fixture\n").unwrap();
    assert!(
        npmrc
            .credential_for_url_checked("https://user:pass@registry.example/team/")
            .is_err()
    );
}
