//! URL security and endpoint contract tests.
//! Test bảo mật URL và hợp đồng endpoint.

use crate::commands::registry::registry_url::{
    RegistryUrlError, binding_url, revoke_token_url, user_url,
};
use url::Url;

use super::super::should_bypass_system_proxy;

#[test]
fn endpoint_builders_preserve_prefix_and_encode_dynamic_segments() {
    let user = user_url("https://registry.example/npm/team/", "alice/admin").unwrap();
    assert_eq!(
        user.as_str(),
        "https://registry.example/npm/team/-/user/org.couchdb.user:alice%2Fadmin"
    );
    let revoke = revoke_token_url("https://registry.example/prefix/", "secret/segment").unwrap();
    assert_eq!(
        revoke.as_str(),
        "https://registry.example/prefix/-/user/token/secret%2Fsegment"
    );
    assert_eq!(
        binding_url("https://registry.example/npm")
            .unwrap()
            .as_str(),
        "https://registry.example/npm/-/v1/trusted-publish/bindings"
    );
}

#[test]
fn registry_url_policy_accepts_https_and_explicit_loopback_http() {
    for registry in [
        "https://registry.example/npm",
        "http://localhost:4315/npm",
        "http://127.0.0.1:4315/npm",
        "http://[::1]:4315/npm",
    ] {
        assert!(user_url(registry, "alice").is_ok(), "{registry}");
    }
}

#[test]
fn registry_url_policy_rejects_insecure_or_ambiguous_bases() {
    for registry in [
        "http://registry.example/npm",
        "ftp://registry.example/npm",
        "https://user:password@registry.example/npm",
        "https://@registry.example/npm",
        "https://registry.example/npm?token=secret",
        "https://registry.example/npm#fragment",
    ] {
        assert!(user_url(registry, "alice").is_err(), "{registry}");
    }
    assert!(binding_url("http://registry.example/npm").is_err());
    assert!(matches!(
        user_url("http://registry.example/npm", "alice"),
        Err(RegistryUrlError::RequiresHttps)
    ));
}

#[test]
fn loopback_endpoints_bypass_ambient_proxies_but_remote_registries_keep_them() {
    for address in [
        "http://localhost:4315/npm",
        "http://127.0.0.1:4315/npm",
        "http://[::1]:4315/npm",
    ] {
        assert!(should_bypass_system_proxy(&Url::parse(address).unwrap()));
    }
    assert!(!should_bypass_system_proxy(
        &Url::parse("https://registry.example/npm").unwrap()
    ));
}
