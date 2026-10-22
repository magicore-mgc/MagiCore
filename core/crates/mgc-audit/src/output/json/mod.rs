//! Stable facade for versioned JSON audit output.
//! Facade ổn định cho output audit JSON có version.

mod envelope;
mod skipped_reasons;
mod status;

pub use self::envelope::{JSON_SCHEMA_VERSION, JsonAuditEnvelope};
