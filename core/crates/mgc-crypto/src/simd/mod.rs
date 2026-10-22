//! SIMD capability reporting API — API báo cáo capability SIMD.
//! Keep module paths stable while wiring focused owners — Giữ module path ổn định và nối các owner chuyên trách.

mod capability;
mod detection;
mod info;

pub use capability::SimdCapability;
pub use detection::detect_simd;
pub use info::simd_info;
