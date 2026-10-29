//! Update-target selection wiring.
//! Wiring chọn target cần update.

#[path = "target_selection.rs"]
mod implementation;

pub(in crate::commands::core::shared) use implementation::{
    InstalledVersions, select_update_targets,
};
