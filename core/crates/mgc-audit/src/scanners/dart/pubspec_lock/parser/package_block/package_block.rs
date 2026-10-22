//! Recognize package headers without reclassifying known lockfile fields.
//! Nhận diện header package mà không phân loại nhầm field lockfile đã biết.

use crate::scanners::dart::pubspec_lock::state::PubspecLockState;

const PACKAGE_FIELD_PREFIXES: [&str; 6] = [
    "version:",
    "source:",
    "description:",
    "sha256:",
    "url:",
    "source-map:",
];

pub(in crate::scanners::dart::pubspec_lock::parser) fn begin_if_header(
    state: &mut PubspecLockState,
    entry: &str,
) -> bool {
    if entry.ends_with(':')
        && !PACKAGE_FIELD_PREFIXES
            .iter()
            .any(|prefix| entry.starts_with(prefix))
    {
        state.current_name = Some(entry[..entry.len() - 1].to_string());
        return true;
    }
    false
}

#[cfg(test)]
#[path = "test/package_block.rs"]
mod tests;
