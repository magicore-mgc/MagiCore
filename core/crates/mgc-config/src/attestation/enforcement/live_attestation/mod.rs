//! Live-marker enforcement wiring / Wiring enforcement marker hiện hữu.
//! This module exports live attestation policy and its private typed errors.

mod error;
mod live_attestation;

pub use live_attestation::enforce_live_attestation;
