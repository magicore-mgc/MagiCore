//! App install language and offline policy wiring.
//! Wiring policy language và offline cho install App.

mod language_gate;

pub(super) use language_gate::{ensure_dispatchable_language, ensure_offline_supported};
