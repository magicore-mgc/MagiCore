//! Wire the public JSON envelope and its focused tests.
//! Nối envelope JSON public và các test tập trung.

#[path = "envelope.rs"]
mod envelope;

pub use self::envelope::{JSON_SCHEMA_VERSION, JsonAuditEnvelope};

#[cfg(test)]
#[path = "test/envelope.rs"]
mod tests;
