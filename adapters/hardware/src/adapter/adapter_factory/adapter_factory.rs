//! Select the hardware adapter when a MagiCore project manifest exists.
//! Chọn adapter hardware khi project có manifest MagiCore.

use super::super::adapter::HardwareAdapter;
use crate::detection::manifest_is_any_mg;
use std::path::Path;

pub fn adapter_for(root: &Path) -> Option<HardwareAdapter> {
    manifest_is_any_mg(root).then_some(HardwareAdapter)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
