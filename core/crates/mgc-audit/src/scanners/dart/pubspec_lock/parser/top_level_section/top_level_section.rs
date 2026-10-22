//! Reset the active package block on each root-level key.
//! Reset block package đang hoạt động mỗi khi có khóa gốc.

use crate::scanners::dart::pubspec_lock::state::PubspecLockState;

pub(in crate::scanners::dart::pubspec_lock::parser) fn reset_for_root_key(
    state: &mut PubspecLockState,
    key: &str,
) {
    state.in_packages = key == "packages:";
    state.current_name = None;
}

#[cfg(test)]
#[path = "test/top_level_section.rs"]
mod tests;
