//! Runtime detection behavior tests — Test hành vi phát hiện runtime.
//! Keep architecture assertions behind their production target cfgs — Đặt assertion theo kiến trúc dưới cfg production tương ứng.

use crate::simd::{SimdCapability, detect_simd};

#[test]
fn detection_is_repeatable_for_a_stable_host() {
    assert_eq!(detect_simd(), detect_simd());
}

#[test]
#[cfg(target_arch = "x86_64")]
fn x86_64_keeps_sse2_baseline_detection() {
    assert!(matches!(
        detect_simd(),
        SimdCapability::SSE2 | SimdCapability::AVX2 | SimdCapability::AVX512
    ));
}

#[test]
#[cfg(target_arch = "aarch64")]
fn aarch64_keeps_neon_baseline_detection() {
    assert_eq!(detect_simd(), SimdCapability::NEON);
}

#[test]
#[cfg(all(target_arch = "arm", target_feature = "neon"))]
fn arm_with_neon_target_feature_reports_neon() {
    assert_eq!(detect_simd(), SimdCapability::NEON);
}
