// Framework seed test wiring — nối các test hợp đồng catalog.
// Keep test bodies in named files next to this owner.
pub(super) use super::FRAMEWORK_SEEDS;

#[path = "catalog.rs"]
mod catalog;
