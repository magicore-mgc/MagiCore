//! SIMD capability summary string — Chuỗi tổng hợp capability SIMD.
//! Preserve the established text and formatting — Giữ nguyên nội dung và format hiện có.

use super::super::{SimdCapability, detect_simd};

/// Return the current SIMD info summary — Trả chuỗi tổng hợp SIMD hiện tại.
pub fn simd_info() -> String {
    let capability: SimdCapability = detect_simd();
    format!(
        "SIMD: {} ({}x performance)",
        capability.name(),
        capability.performance_multiplier()
    )
}
