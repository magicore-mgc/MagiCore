//! Stable lockfile verification, status, and provenance API.
//! API ổn định cho xác minh lockfile, status và provenance.

mod legacy;
mod ownership_completeness;
mod status;
mod status_message;
mod tamper_check;
mod v4;
mod verifier;

pub use self::ownership_completeness::verify_ownership_completeness;
pub use self::status::VerificationStatus;
pub use self::status_message::verification_status_message;
pub use self::tamper_check::is_lockfile_tampered;
pub use self::verifier::{verify_lockfile, verify_lockfile_with_trust};
