//! Process-wide warning policy / Policy cảnh báo toàn process.
//! Emits at most one enforcement warning and exposes a pure claim seam.

use std::sync::OnceLock;

static WARNED_ONCE: OnceLock<()> = OnceLock::new();

pub(in crate::attestation) fn warn_once(message: &str) {
    if claim_once(&WARNED_ONCE) {
        eprintln!("{message}");
    }
}

fn claim_once(gate: &OnceLock<()>) -> bool {
    gate.set(()).is_ok()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
