//! Mutable state for parsing the generated two-level Pub lockfile subset.
//! State mutable để parse tập con lockfile Pub hai cấp do pub sinh.

use crate::scanners::osv::OsvPin;

#[derive(Default)]
pub(in crate::scanners::dart::pubspec_lock) struct PubspecLockState {
    pub(in crate::scanners::dart::pubspec_lock) in_packages: bool,
    pub(in crate::scanners::dart::pubspec_lock) current_name: Option<String>,
    pub(in crate::scanners::dart::pubspec_lock) pins: Vec<OsvPin>,
    pub(in crate::scanners::dart::pubspec_lock) skipped: Vec<String>,
}

impl PubspecLockState {
    pub(in crate::scanners::dart::pubspec_lock) fn finish(self) -> (Vec<OsvPin>, Vec<String>) {
        (self.pins, self.skipped)
    }
}

#[cfg(test)]
#[path = "test/state.rs"]
mod tests;
