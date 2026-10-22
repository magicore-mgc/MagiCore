//! CAS generation abort module wiring and phase exports.
//! Wiring module abort CAS generation và export các phase.
mod abort;
#[path = "delete_claims/mod.rs"]
mod delete_claims;
#[path = "retire_marker/mod.rs"]
mod retire_marker;
#[path = "validate_abort_state/mod.rs"]
mod validate_abort_state;
