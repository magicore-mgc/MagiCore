//! Interrupted mutation recovery wiring.
//! Wiring recovery mutation gián đoạn.

#[path = "recovery.rs"]
mod implementation;

pub(crate) use implementation::ensure_no_pending_remove_journal;
pub(crate) use implementation::recover_interrupted_remove;
