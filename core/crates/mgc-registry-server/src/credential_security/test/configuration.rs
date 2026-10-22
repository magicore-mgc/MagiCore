#[test]
fn configuration_debug_redacts_tokens_and_url_credentials() {
    let config = crate::RegistryServerConfig {
        host: "127.0.0.1".into(),
        port: 0,
        store_dir: "registry-test".into(),
        admin_token: Some("private-admin-value".into()),
        max_body_size: 1024,
        rate_limit_rps: 5,
        upstream: Some("https://user:private-upstream-value@example.test".into()),
        storage: Some("s3://user:private-storage-value@example.test".into()),
        oidc_issuers: vec![],
        oidc_audience: None,
        trusted_proxy_ips: vec![],
    };
    let debug = format!("{config:?}");
    for secret in [
        "private-admin-value",
        "private-upstream-value",
        "private-storage-value",
    ] {
        assert!(!debug.contains(secret));
    }
    assert!(debug.contains("[REDACTED]"));
}
