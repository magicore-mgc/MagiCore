// Framework catalog wiring — nối quan hệ framework với dữ liệu seed.
// Keep catalog sequence stable for existing manifest selection.
use super::*;
#[path = "framework_catalog.rs"]
mod implementation;
#[path = "seeds/mod.rs"]
mod seeds;
pub(super) use implementation::*;
pub(super) use seeds::FRAMEWORK_SEEDS;
