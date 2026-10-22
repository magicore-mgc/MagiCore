//! Fail-closed sha256 verification tests for Go artifacts.
//! Test xác minh sha256 fail-closed cho artifact Go.

use super::super::*;

fn entry_with_sha256(expected: &str) -> ResolvedEntry {
    ResolvedEntry {
        name: "example.test/pkg".to_string(),
        version: "1.2.3".to_string(),
        deps: Vec::new(),
        artifact_url: "https://proxy.invalid/artifact.zip".to_string(),
        sha256: expected.to_string(),
        extra_markers: Vec::new(),
    }
}

#[test]
fn sha256_verification_accepts_case_and_rejects_changed_bytes() {
    let protocol = GoModProtocol::with_sum_base("https://proxy.invalid", "https://sum.invalid");
    let bytes = b"artifact";
    let entry = entry_with_sha256(&sha256_hex(bytes).to_ascii_uppercase());
    assert!(protocol.verify_artifact(&entry, bytes).is_ok());
    assert!(matches!(
        protocol.verify_artifact(&entry, b"changed"),
        Err(MgError::Integrity(_))
    ));
}

#[test]
fn missing_integrity_evidence_fails_closed() {
    let protocol = GoModProtocol::with_sum_base("https://proxy.invalid", "https://sum.invalid");
    let mut entry = entry_with_sha256("");
    entry.extra_markers.clear();
    assert!(matches!(
        protocol.verify_artifact(&entry, b"artifact"),
        Err(MgError::Integrity(_))
    ));
}
