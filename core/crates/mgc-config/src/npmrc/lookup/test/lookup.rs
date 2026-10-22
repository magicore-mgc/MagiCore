#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn normalizes_host_and_prefers_scoped_registry() {
    assert_eq!(
        NpmRc::normalize_host("//registry.example/"),
        "registry.example"
    );
    let npmrc =
        NpmRc::parse("registry=https://default.example/\n@team:registry=https://team.example/\n")
            .unwrap();
    assert_eq!(
        npmrc.registry_for(Some("@team")).as_deref(),
        Some("https://team.example/")
    );
    assert_eq!(
        npmrc.registry_for(Some("@other")).as_deref(),
        Some("https://default.example/")
    );
}
