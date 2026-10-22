//! Runtime CPU feature detection — Phát hiện feature CPU runtime.
//! Preserve architecture gates and feature priority — Giữ cfg kiến trúc và thứ tự ưu tiên feature.

use super::super::SimdCapability;

/// Detect available SIMD capability at runtime — Phát hiện capability SIMD hiện có lúc runtime.
pub fn detect_simd() -> SimdCapability {
    // BLAKE3 performs its own SIMD dispatch; this function only reports host features.
    // BLAKE3 tự dispatch SIMD; hàm này chỉ báo feature của host.

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            return SimdCapability::AVX512;
        }
        if is_x86_feature_detected!("avx2") {
            return SimdCapability::AVX2;
        }
        if is_x86_feature_detected!("sse2") {
            return SimdCapability::SSE2;
        }
    }

    #[cfg(target_arch = "x86")]
    {
        if is_x86_feature_detected!("avx2") {
            return SimdCapability::AVX2;
        }
        if is_x86_feature_detected!("sse2") {
            return SimdCapability::SSE2;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        // NEON is a baseline feature on AArch64.
        // NEON là feature nền tảng của AArch64.
        return SimdCapability::NEON;
    }

    #[cfg(all(target_arch = "arm", target_feature = "neon"))]
    {
        return SimdCapability::NEON;
    }

    // Fallback when the host exposes no supported feature — Fallback khi host không có feature hỗ trợ.
    #[allow(unreachable_code)]
    SimdCapability::None
}
