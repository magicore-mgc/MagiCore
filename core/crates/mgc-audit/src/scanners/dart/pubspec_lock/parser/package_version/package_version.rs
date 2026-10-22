//! Convert the current package's version field into an OSV pin.
//! Chuyển field version của package hiện tại thành pin OSV.

use crate::scanners::dart::pubspec_lock::state::PubspecLockState;
use crate::scanners::osv::OsvPin;

pub(in crate::scanners::dart::pubspec_lock::parser) fn record_if_version(
    state: &mut PubspecLockState,
    entry: &str,
) {
    let Some(rest) = entry.strip_prefix("version:") else {
        return;
    };
    let Some(name) = state.current_name.clone() else {
        return;
    };
    let version = rest.trim().trim_matches('"');
    if version.is_empty() {
        state.skipped.push(format!("{name}: empty version pin"));
    } else {
        state.pins.push(OsvPin {
            name,
            version: version.to_string(),
            ecosystem: "Pub",
        });
    }
}

#[cfg(test)]
#[path = "test/package_version.rs"]
mod tests;
