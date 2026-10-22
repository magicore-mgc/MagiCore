//! Behavior cases for model weight routing and pickle delegation.
//! Test hành vi cho định tuyến model weight và chuyển tiếp sang pickle.

use super::support::tmp;
use crate::audit::scanner::scan_weights;

#[test]
fn test_scan_weights_large() {
    let tmp = tmp();
    let weights = tmp.path().join("huge.bin");

    // This fixture has small contents; it does not create a 101 GiB file.
    // Fixture này có nội dung nhỏ; nó không tạo file 101 GiB thật.
    std::fs::write(&weights, b"small file").unwrap();

    let findings = scan_weights(&weights).unwrap();
    // A small file produces no size finding.
    // File nhỏ không tạo size finding.
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
