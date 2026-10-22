//! Wires project state capture and restore-source overlap owners.
//! Nối owner chụp project state và kiểm tra overlap nguồn restore.

mod project_state;

pub(super) use project_state::{ProjectState, inspect_project_state, validate_restore_source};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
