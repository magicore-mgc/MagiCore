//! Emit the process-local warning for the explicit untrusted bypass.
//! In cảnh báo theo process khi dùng bypass untrusted tường minh.

use std::cell::Cell;
use std::sync::OnceLock;

pub(in crate::supply_chain) fn warn_once_if_untrusted(allow_untrusted: bool) {
    static WARNED: OnceLock<()> = OnceLock::new();
    warn_once_if_untrusted_with(allow_untrusted, &WARNED);
}

fn warn_once_if_untrusted_with(allow_untrusted: bool, warned: &OnceLock<()>) -> bool {
    if !allow_untrusted {
        return false;
    }

    let emitted = Cell::new(false);
    warned.get_or_init(|| {
        eprintln!(
            "WARN: [magicore] WARNING: MAGICORE_ALLOW_UNTRUSTED=1 — supply-chain guards\n   \
             (24h quarantine + no-downgrade) are BYPASSED for this process."
        );
        emitted.set(true);
    });
    emitted.get()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
