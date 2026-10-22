#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn parses_supported_entries_and_ignores_comments_or_unknown_keys() {
    let npmrc = NpmRc::parse(concat!(
        "# comment\n; another comment\ncache=/tmp\n",
        "registry=https://registry.example/\n",
        "@team:registry=https://team.example/\n",
        "//registry.example/:_authToken=fixture-token\n",
    ))
    .unwrap();
    assert_eq!(npmrc.registry.as_deref(), Some("https://registry.example/"));
    assert_eq!(
        npmrc.registry_for(Some("@team")).as_deref(),
        Some("https://team.example/")
    );
    assert_eq!(
        npmrc.token_for("registry.example").map(String::as_str),
        Some("fixture-token")
    );
}
