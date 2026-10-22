//! SRI computation tests — Test tính SRI.
//! Preserve BLAKE3 and Base64 output — Giữ output BLAKE3 và Base64.

use crate::integrity::{IntegrityVerifier, SriHash};

#[test]
fn computes_blake3_sri_value() {
    let sri = IntegrityVerifier::compute(b"compute owner");
    assert_eq!(sri.algorithm, "blake3");
    assert_eq!(
        SriHash::parse(&sri.to_string()).expect("test operation must succeed"),
        sri
    );
}
