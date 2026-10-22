#![allow(clippy::unwrap_used)]
//! Fetch error compatibility tests. / Kiểm thử tương thích lỗi fetch.

use super::super::*;
use mgc_types::MgError;

#[test]
fn missing_registry_keeps_network_category_and_message() {
    let legacy = MgError::from(FetchError::MissingRegistry {
        package: "widget".to_string(),
    });

    let MgError::Network(message) = legacy else {
        panic!("missing registry must remain a network error");
    };
    assert_eq!(
        message,
        "no registry URL provided for 'widget' and no cache hit"
    );
}

#[test]
fn cache_error_retains_local_source_and_store_category() {
    let error = FetchError::LocalCacheRead(anyhow::anyhow!("cache read failed"));
    assert!(std::error::Error::source(&error).is_some());

    let legacy = MgError::from(error);
    let MgError::Store(message) = legacy else {
        panic!("cache reads must remain store errors");
    };
    assert_eq!(message, "cache read failed");
}
