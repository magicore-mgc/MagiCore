//! Warn once when explicit policy accepts unavailable publish time.
//! Cảnh báo một lần khi policy tường minh chấp nhận thiếu publish time.

use std::cell::Cell;
use std::sync::OnceLock;

pub(in crate::supply_chain) fn warn_once_if_missing_publish_time() {
    static WARNED: OnceLock<()> = OnceLock::new();
    warn_once_if_missing_publish_time_with(&WARNED);
}

fn warn_once_if_missing_publish_time_with(warned: &OnceLock<()>) -> bool {
    let emitted = Cell::new(false);
    warned.get_or_init(|| {
        eprintln!(
            "WARN: [magicore] [security] allow_missing_time=true accepted at least one \
             package without a valid registry publish timestamp."
        );
        emitted.set(true);
    });
    emitted.get()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
