//! Boundary cases for SafeTensors header validation.
//! Test ranh giới cho kiểm tra header SafeTensors.

use super::support::tmp;
use crate::audit::scanner::scan_safetensors;

#[test]
fn test_scan_safetensors_valid() {
    let tmp = tmp();
    let st = tmp.path().join("model.safetensors");

    // Create minimal valid SafeTensors data: an eight-byte length and JSON.
    // Tạo dữ liệu SafeTensors tối thiểu: độ dài 8 byte và JSON.
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
