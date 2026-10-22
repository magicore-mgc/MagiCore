//! Reject insecure non-loopback URL before network access.
//! Từ chối URL HTTP không loopback trước khi truy cập mạng.

#![allow(clippy::unwrap_used)]

use super::refresh_manifest_from_url;
use crate::commands::policy::manifest_distribution::error::PolicyManifestDistributionError;

#[test]
fn invalid_url_error_keeps_parse_source_and_display() {
    let source = url::Url::parse("http://[").unwrap_err();
    let detail = source.to_string();
    let error = PolicyManifestDistributionError::InvalidUrl {
        detail: detail.clone(),
        source,
    };

    assert_eq!(
        error.to_string(),
        format!("policy manifest URL invalid: {detail}")
    );
    assert!(std::error::Error::source(&error).is_some());
}

#[tokio::test]
async fn refuses_plain_http_outside_loopback() {
    let directory = tempfile::tempdir().unwrap();
    let cache = directory.path().join("exceptions.json");
    let error = refresh_manifest_from_url(
        "http://registry.example.invalid/policy.json",
        &["deadbeefdeadbeef".to_string()],
        &cache,
    )
    .await
    .unwrap_err();

    assert!(error.to_string().contains("HTTPS"));
    assert!(!cache.exists());
}
