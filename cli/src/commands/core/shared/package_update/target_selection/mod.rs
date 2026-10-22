//! Update-target selection wiring.
//! Wiring chọn target cần update.

#[path = "target_selection.rs"]
mod implementation;

pub(crate) use implementation::select_update_targets;
