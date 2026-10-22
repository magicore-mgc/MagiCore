//! Capability label, estimate and serde tests — Test nhãn, ước lượng và serde capability.
//! Freeze compatibility values without claiming measured throughput — Cố định value tương thích, không khẳng định thông lượng đã đo.

use crate::simd::SimdCapability;

#[test]
fn preserves_names_and_display_multipliers() {
    let cases = [
        (SimdCapability::None, "None (fallback)", 1.0),
        (SimdCapability::SSE2, "SSE2", 2.0),
        (SimdCapability::AVX2, "AVX2", 4.0),
        (SimdCapability::AVX512, "AVX-512", 8.0),
        (SimdCapability::NEON, "NEON", 3.0),
    ];

    for (capability, name, multiplier) in cases {
        assert_eq!(capability.name(), name);
        assert_eq!(capability.performance_multiplier(), multiplier);
    }
}

#[test]
fn preserves_serde_roundtrip_for_each_variant() {
    for capability in [
        SimdCapability::None,
        SimdCapability::SSE2,
        SimdCapability::AVX2,
        SimdCapability::AVX512,
        SimdCapability::NEON,
    ] {
        let encoded = serde_json::to_string(&capability).expect("test operation must succeed");
        let decoded: SimdCapability =
            serde_json::from_str(&encoded).expect("test operation must succeed");
        assert_eq!(decoded, capability);
    }
}
