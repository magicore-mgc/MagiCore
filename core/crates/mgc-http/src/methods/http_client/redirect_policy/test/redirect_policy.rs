//! SEC-8/G5 redirect policy tests: hop cap, cross-origin denial, redaction.
//! Test policy redirect SEC-8/G5: giới hạn hop, chặn khác origin, che URL.

use super::super::redirect_policy::{
    DEFAULT_MAX_REDIRECTS, redact_url_for_test, same_origin_for_test,
};
use reqwest::Url;

fn url(raw: &str) -> Url {
    Url::parse(raw).expect("test URL must parse")
}

#[test]
fn same_host_same_scheme_is_allowed() {
    let initial = url("https://registry.example.com/pkg");
    assert!(same_origin_for_test(&url("https://registry.example.com/other"), &[initial]));
}

#[test]
fn cross_host_redirect_is_not_same_origin() {
    let initial = url("https://registry.example.com/pkg");
    assert!(!same_origin_for_test(
        &url("https://attacker.example.net/pkg"),
        std::slice::from_ref(&initial)
    ));
    assert!(!same_origin_for_test(
        &url("https://registry.example.com.evil.net/pkg"),
        &[initial]
    ));
}

#[test]
fn different_explicit_port_is_not_same_origin() {
    let initial = url("https://registry.example.com:8443/pkg");
    assert!(!same_origin_for_test(
        &url("https://registry.example.com/pkg"),
        std::slice::from_ref(&initial)
    ));
    assert!(!same_origin_for_test(
        &url("https://registry.example.com:9443/pkg"),
        &[initial]
    ));
}

#[test]
fn scheme_upgrade_that_changes_the_default_port_is_allowed() {
    // http:80 -> https:443 on one host is the only accepted scheme change.
    // (http:80 -> https:443 trên một host là thay đổi scheme duy nhất được nhận.)
    let initial = url("http://registry.example.com/pkg");
    assert!(same_origin_for_test(
        &url("https://registry.example.com/pkg"),
        &[initial]
    ));
}

#[test]
fn https_downgrade_to_http_is_never_same_origin() {
    let initial = url("https://registry.example.com/pkg");
    assert!(!same_origin_for_test(&url("http://registry.example.com/pkg"), &[initial]));
}

#[test]
fn downgrade_is_judged_against_the_chain_start() {
    // The origin identity is the initial URL, so an https-first chain that
    // later downgrades to http is rejected for the rest of the chain.
    // (Origin được so với URL đầu, nên chuỗi bắt đầu bằng https rồi hạ xuống
    // http bị chặn cho hết chuỗi còn lại.)
    let https = url("https://registry.example.com/pkg");
    let http = url("http://registry.example.com/pkg");
    assert!(!same_origin_for_test(&http, std::slice::from_ref(&https)));
    let http_start = url("http://registry.example.com/pkg");
    assert!(same_origin_for_test(
        &url("https://registry.example.com/next"),
        std::slice::from_ref(&http_start)
    ));
}

#[test]
fn empty_chain_is_never_same_origin() {
    assert!(!same_origin_for_test(&url("https://registry.example.com/pkg"), &[]));
}

#[test]
fn redacted_url_drops_userinfo_and_query_credentials() {
    let safe = redact_url_for_test(&url(
        "https://user:token-value@registry.example.com/pkg?token=leaked#frag",
    ));
    assert!(!safe.contains("token-value"), "userinfo leaked: {safe}");
    assert!(!safe.contains("leaked"), "query leaked: {safe}");
    assert!(safe.starts_with("https://registry.example.com/pkg"));
}

#[test]
fn default_hop_cap_is_bounded_and_nonzero() {
    assert_eq!(DEFAULT_MAX_REDIRECTS, 5);
}
