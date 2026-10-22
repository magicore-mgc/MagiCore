//! Web registry endpoint and URL policy integration tests.
//! Test integration endpoint và URL policy Web registry.
#![allow(clippy::unwrap_used)]

use mgc::commands::web_registry_config as web_registry_api;

#[test]
fn builds_search_endpoint_from_default_base() {
    let url = web_registry_api::search_endpoint(
        web_registry_api::DEFAULT_WEB_REGISTRY_URL,
        "react vite",
        20,
        40,
    )
    .unwrap();
    assert_eq!(
        url,
        "https://registry.npmjs.org/-/v1/search?text=react+vite&size=20&from=40"
    );
}

#[test]
fn preserves_registry_path_prefix() {
    let url = web_registry_api::advisory_bulk_endpoint("https://registry.example.com/npm").unwrap();
    assert_eq!(
        url,
        "https://registry.example.com/npm/-/npm/v1/security/advisories/bulk"
    );
}

#[test]
fn rejects_plain_http_remote_registry() {
    let err = web_registry_api::join_registry_path("http://registry.example.com", "-/v1/search")
        .unwrap_err();
    assert!(err.to_string().contains("must use HTTPS"));
}

#[test]
fn allows_plain_http_loopback_registry() {
    let url = web_registry_api::join_registry_path("http://127.0.0.1:4315", "-/v1/search").unwrap();
    assert_eq!(url.to_string(), "http://127.0.0.1:4315/-/v1/search");
}

#[test]
fn accepts_ipv6_and_noncanonical_ipv4_loopback_addresses() {
    for base in ["http://[::1]:4315", "http://127.0.0.2:4315"] {
        assert!(web_registry_api::join_registry_path(base, "-/v1/search").is_ok());
    }
}

#[test]
fn rejects_url_credentials_query_and_fragment_without_echoing_secrets() {
    for base in [
        "https://user:secret@registry.example.com/npm",
        "https://registry.example.com/npm?token=secret",
        "https://registry.example.com/npm#secret",
    ] {
        let error = web_registry_api::join_registry_path(base, "-/v1/search").unwrap_err();
        let diagnostic = format!("{error:#}");
        assert!(!diagnostic.contains("secret"), "{diagnostic}");
    }
}
