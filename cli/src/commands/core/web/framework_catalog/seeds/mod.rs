// Framework seed catalog wiring — nối dữ liệu seed framework.
// Keep declarations and order unchanged.
#[path = "seeds.rs"]
mod implementation;
pub(in crate::commands::core::web::implementation) use implementation::FRAMEWORK_SEEDS;
