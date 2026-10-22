//! SIMD info format compatibility test — Test tương thích format thông tin SIMD.
//! Compare against the existing output template — So sánh với template output hiện tại.

use crate::simd::{detect_simd, simd_info};

#[test]
fn preserves_host_summary_format() {
    let capability = detect_simd();
    let expected = format!(
        "SIMD: {} ({}x performance)",
        capability.name(),
        capability.performance_multiplier()
    );
    assert_eq!(simd_info(), expected);
}
