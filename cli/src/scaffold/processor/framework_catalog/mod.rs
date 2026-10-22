// Framework catalog wiring — separate Web metadata from shared core defaults.
// Wiring catalog framework — tách metadata Web khỏi default dùng chung các core.
use super::*;

mod core_default_framework;
#[path = "framework_catalog.rs"]
mod implementation;
mod web_framework_catalog;

pub(in crate::scaffold::processor) use web_framework_catalog::WEB_FRAMEWORKS;
