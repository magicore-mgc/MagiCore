//! CocoaPods shared error category and exact-message tests.
//! Test nhóm lỗi dùng chung và thông điệp chính xác của CocoaPods.

use super::*;
use mgc_types::MgError;

#[test]
fn checksum_errors_remain_integrity_failures() {
    let invalid: MgError = CocoaPodsClientError::InvalidChecksum {
        name: "PodA".to_string(),
        expected: "bad".to_string(),
    }
    .into_mg_error();
    assert!(
        matches!(invalid, MgError::Integrity(message) if message == "Podfile.lock checksum for PodA is not a sha1 hex digest: 'bad'")
    );

    let mismatch: MgError = CocoaPodsClientError::ChecksumMismatch {
        name: "PodA".to_string(),
        version: "1.0.0".to_string(),
        expected: "expected".to_string(),
        actual: "actual".to_string(),
    }
    .into_mg_error();
    assert!(
        matches!(mismatch, MgError::Integrity(message) if message == "podspec sha1 mismatch for PodA 1.0.0: Podfile.lock says expected, CDN spec is actual (fail-closed)")
    );
}

#[test]
fn request_status_and_json_errors_keep_their_shared_categories() {
    let request: MgError = CocoaPodsClientError::RequestFailed {
        url: "https://cdn.example/spec".to_string(),
        source: anyhow::anyhow!("connection refused"),
    }
    .into_mg_error();
    assert!(
        matches!(request, MgError::Network(message) if message == "GET https://cdn.example/spec failed: connection refused")
    );

    let body: MgError = CocoaPodsClientError::BodyReadFailed {
        url: "https://cdn.example/spec".to_string(),
        source: anyhow::anyhow!("body stream closed"),
    }
    .into_mg_error();
    assert!(
        matches!(body, MgError::Network(message) if message == "read https://cdn.example/spec body failed: body stream closed")
    );

    let status: MgError = CocoaPodsClientError::HttpStatus {
        url: "https://cdn.example/spec".to_string(),
        status: "404 Not Found".to_string(),
    }
    .into_mg_error();
    assert!(
        matches!(status, MgError::Network(message) if message == "GET https://cdn.example/spec returned 404 Not Found")
    );

    let json: MgError = CocoaPodsClientError::InvalidJson {
        name: "PodA".to_string(),
        version: "1.0.0".to_string(),
        source: serde_json::from_str::<serde_json::Value>("{")
            .expect_err("test operation must fail"),
    }
    .into_mg_error();
    assert!(
        matches!(json, MgError::Other(message) if message.starts_with("parse podspec JSON for PodA 1.0.0 failed:"))
    );
}
