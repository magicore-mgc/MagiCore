//! Wiring for claim admission test implementations.
//! Nối implementation test của claim admission.

#[path = "claim_admission.rs"]
mod claim_admission;
pub(crate) use claim_admission::*;
