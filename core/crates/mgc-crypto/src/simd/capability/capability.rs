//! Public SIMD capability value and labels — Value capability SIMD public và nhãn.
//! Keep serde shape and display estimates stable — Giữ shape serde và ước lượng hiển thị ổn định.

use serde::{Deserialize, Serialize};

/// SIMD capability detected on this host — Capability SIMD được phát hiện trên host này.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimdCapability {
    /// No SIMD capability detected — Không phát hiện capability SIMD.
    None,
    /// SSE2 capability — Năng lực xử lý SSE2.
    SSE2,
    /// AVX2 capability — Năng lực xử lý AVX2.
    AVX2,
    /// AVX-512 capability — Năng lực xử lý AVX-512.
    AVX512,
    /// NEON capability — Năng lực xử lý NEON.
    NEON,
}

impl SimdCapability {
    /// Return the stable human-readable capability name — Trả tên capability dễ đọc ổn định.
    pub fn name(&self) -> &'static str {
        match self {
            SimdCapability::None => "None (fallback)",
            SimdCapability::SSE2 => "SSE2",
            SimdCapability::AVX2 => "AVX2",
            SimdCapability::AVX512 => "AVX-512",
            SimdCapability::NEON => "NEON",
        }
    }

    /// Return the legacy display multiplier estimate — Trả multiplier ước lượng legacy để hiển thị.
    pub fn performance_multiplier(&self) -> f64 {
        match self {
            SimdCapability::None => 1.0,
            SimdCapability::SSE2 => 2.0,
            SimdCapability::AVX2 => 4.0,
            SimdCapability::AVX512 => 8.0,
            SimdCapability::NEON => 3.0,
        }
    }
}
