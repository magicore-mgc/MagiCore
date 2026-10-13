#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::*;
use tempfile::TempDir;

fn tmp() -> TempDir {
    TempDir::new().unwrap()
}

#[test]
fn test_scan_pickle_safe() {
    let tmp = tmp();
    let pkl = tmp.path().join("safe.pkl");
    std::fs::write(&pkl, b"safe pickle data").unwrap();

    let findings = scan_pickle(&pkl).unwrap();
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_scan_pickle_dangerous() {
    let tmp = tmp();
    let pkl = tmp.path().join("evil.pkl");
    std::fs::write(&pkl, b"os.system('rm -rf /')").unwrap();

    let findings = scan_pickle(&pkl).unwrap();
    assert!(!findings.is_empty());
    assert_eq!(findings[0].severity, super::super::Severity::Critical);
}

#[test]
fn test_scan_safetensors_valid() {
    let tmp = tmp();
    let st = tmp.path().join("model.safetensors");

    // Create minimal valid safetensors: 8-byte header + JSON
    let mut data = vec![0u8; 8];
    let json = b"{}";
    let header_len = json.len() as u64;
    data[0..8].copy_from_slice(&header_len.to_le_bytes());
    data.extend_from_slice(json);

    std::fs::write(&st, data).unwrap();

    let findings = scan_safetensors(&st).unwrap();
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_scan_safetensors_invalid() {
    let tmp = tmp();
    let st = tmp.path().join("bad.safetensors");
    std::fs::write(&st, b"short").unwrap();

    let findings = scan_safetensors(&st).unwrap();
    assert!(!findings.is_empty());
}

#[test]
fn test_scan_weights_large() {
    let tmp = tmp();
    let weights = tmp.path().join("huge.bin");

    // Create 101GB file metadata (stub - don't actually write)
    std::fs::write(&weights, b"small file").unwrap();

    let findings = scan_weights(&weights).unwrap();
    // Small file - no findings
    assert_eq!(findings.len(), 0);
}

#[test]
fn scan_weights_detects_pickle_payload_without_one_specific_header() {
    let tmp = tmp();
    let weights = tmp.path().join("checkpoint.pth");
    std::fs::write(&weights, b"legacy checkpoint cos\nsystem\ncommand\n.").unwrap();

    let findings = scan_weights(&weights).unwrap();

    assert!(
        findings
            .iter()
            .any(|finding| finding.category == "pickle-exploit"),
        "pickle signatures must be checked even when the file does not start with one exact protocol header"
    );
}

#[test]
fn scan_pickle_keeps_signatures_split_across_read_chunks() {
    let tmp = tmp();
    let weights = tmp.path().join("boundary.pth");
    let mut payload = vec![b'a'; 64 * 1024 - 4];
    payload.extend_from_slice(b"cos\nsystem\ncommand\n.");
    std::fs::write(&weights, payload).unwrap();

    let findings = scan_weights(&weights).unwrap();

    assert!(
        findings
            .iter()
            .any(|finding| finding.category == "pickle-exploit")
    );
}

#[test]
fn scan_safetensors_rejects_maximum_header_length_without_overflow() {
    let tmp = tmp();
    let path = tmp.path().join("oversized.safetensors");
    std::fs::write(&path, u64::MAX.to_le_bytes()).unwrap();

    let findings = scan_safetensors(&path).unwrap();

    assert!(
        findings
            .iter()
            .any(|finding| finding.category == "safetensors-header"),
        "oversized metadata must be reported without arithmetic overflow"
    );
}

#[test]
fn test_contains_bytes() {
    assert!(contains_bytes(b"hello world", b"world"));
    assert!(!contains_bytes(b"hello world", b"foo"));
    assert!(contains_bytes(b"os.system", b"os.system"));
}
