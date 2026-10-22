//! Preserve current advisory-mapping skips for non-hosted sources.
//! Giữ cách bỏ qua mapping advisory hiện có cho nguồn không-hosted.

use crate::scanners::dart::pubspec_lock::state::PubspecLockState;

pub(in crate::scanners::dart::pubspec_lock::parser) fn record_if_unsupported(
    state: &mut PubspecLockState,
    entry: &str,
) {
    let Some(rest) = entry.strip_prefix("source:") else {
        return;
    };
    let source = rest.trim();
    if let Some(name) = state.current_name.clone()
        && source != "hosted"
        && !source.is_empty()
    {
        state
            .skipped
            .push(format!("{name}: source '{source}' has no advisory mapping"));
    }
}

#[cfg(test)]
#[path = "test/package_source.rs"]
mod tests;
