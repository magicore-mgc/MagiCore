//! signing owner wiring.
//! Wiring owner Ký và lưu attestation với metadata thời gian/version hiện có.
mod signing;
pub use signing::attester_version;
pub(super) use signing::sign_and_store;
