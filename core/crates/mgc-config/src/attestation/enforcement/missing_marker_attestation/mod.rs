//! Missing-marker enforcement wiring / Wiring enforcement marker thiếu.
//! Exposes missing-marker attestation behavior with local typed errors.

mod error;
mod missing_marker_attestation;

pub use missing_marker_attestation::enforce_missing_marker_attestation;
