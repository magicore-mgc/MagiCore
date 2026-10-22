//! Cache target and path selection owner wiring.
//! File này chỉ nối module target_selection và export API nội bộ.

mod target_selection;

pub(super) use target_selection::cache_entries;
#[cfg(test)]
pub(super) use target_selection::{find_cargo_workspace_root, shared_cache_path};
