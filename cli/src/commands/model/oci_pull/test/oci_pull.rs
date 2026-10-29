#![allow(clippy::unwrap_used)]

// `#[path]` registration puts this file inside the test module, so the parent
// is `oci_pull` itself.
// (`#[path]` đặt file này trong module test nên parent chính là `oci_pull`.)
use super::*;
use crate::commands::model::oci_pull::OciPullError;

/// PAR-12: plaintext is only acceptable for a loopback registry.
/// (PAR-12: plaintext chỉ chấp nhận cho registry loopback.)
#[test]
fn loopback_is_the_only_plaintext_registry() {
    for host in ["127.0.0.1", "localhost", "[::1]", "127.0.0.1:5000"] {
        assert!(is_loopback_registry(host), "{host} is loopback");
    }
    for host in ["ghcr.io", "registry.example.com", "10.0.0.5"] {
        assert!(!is_loopback_registry(host), "{host} must use TLS");
    }
}

/// PAR-12: the refusal names both remedies, so a user knows how to proceed.
/// (PAR-12: lời từ chối nêu cả hai cách để user biết đường làm tiếp.)
#[test]
fn unpinned_reference_error_explains_both_remedies() {
    let error = OciPullError::UnpinnedReference("ghcr.io/acme/model".to_string());
    let message = error.to_string();
    assert!(message.contains("repo:tag"), "{message}");
    assert!(message.contains("repo@sha256:"), "{message}");
}
