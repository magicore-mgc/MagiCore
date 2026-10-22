//! Enforcement policy facade / Facade policy enforcement.
//! Exposes attestation checks while keeping policy implementations isolated.

#[path = "live_attestation/mod.rs"]
mod live_attestation;
#[path = "missing_marker_attestation/mod.rs"]
mod missing_marker_attestation;
#[path = "warning/mod.rs"]
mod warning;

pub use live_attestation::enforce_live_attestation;
pub use missing_marker_attestation::enforce_missing_marker_attestation;
pub(super) use warning::warn_once;
