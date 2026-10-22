// Optional feature package data wiring — nối catalog package feature.
// Data order remains unchanged for manifest processing.
#[path = "feature_catalog.rs"]
mod implementation;
pub(super) use implementation::FEATURE_PACKAGE_GROUPS;
