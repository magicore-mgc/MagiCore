// Package-info error behavior tests.
// Test hành vi lỗi của lệnh thông tin package.

#[cfg(feature = "web")]
use std::error::Error as _;

use super::InfoError;

#[cfg(not(feature = "web"))]
#[test]
fn no_web_adapter_keeps_the_existing_diagnostic() {
    let error = InfoError::NoWebAdapter;
    assert_eq!(
        error.to_string(),
        "package info currently requires the web core registry adapter, which is not included in this build"
    );
}

#[cfg(feature = "web")]
#[test]
fn registry_failure_keeps_text_and_source_chain() {
    let source = anyhow::anyhow!("connection reset");
    let error = InfoError::RegistryFetchFailed { source };
    assert_eq!(
        error.to_string(),
        "Could not fetch package info from registry: connection reset"
    );
    assert_eq!(error.source().unwrap().to_string(), "connection reset");
}
