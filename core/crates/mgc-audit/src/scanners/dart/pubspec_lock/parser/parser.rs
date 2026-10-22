//! Ordered line traversal for the generated Pub lockfile subset.
//! Duyệt dòng theo thứ tự cho tập con lockfile Pub do pub sinh.

use mgc_types::MgResult;

use super::super::state::PubspecLockState;
use super::{package_block, package_source, package_version, top_level_section};

pub(in crate::scanners::dart::pubspec_lock) fn parse(
    raw: &str,
) -> MgResult<(Vec<crate::scanners::osv::OsvPin>, Vec<String>)> {
    let mut state = PubspecLockState::default();

    for line in raw.lines() {
        let trimmed_end = line.trim_end();
        if !line.starts_with(' ') && !trimmed_end.is_empty() {
            top_level_section::reset_for_root_key(&mut state, trimmed_end);
            continue;
        }
        if !state.in_packages {
            continue;
        }

        let entry = trimmed_end.trim_start();
        if package_block::begin_if_header(&mut state, entry) {
            continue;
        }
        package_version::record_if_version(&mut state, entry);
        package_source::record_if_unsupported(&mut state, entry);
    }

    Ok(state.finish())
}

#[cfg(test)]
#[path = "test/parser.rs"]
mod tests;
