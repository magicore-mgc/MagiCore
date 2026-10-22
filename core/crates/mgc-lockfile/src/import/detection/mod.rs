//! Lockfile discovery and trust checks — nhận diện và kiểm tra trust lockfile.

mod detection;
pub use detection::{check_trust_downgrade_risk, detect_legacy_lockfiles};
