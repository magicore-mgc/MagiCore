//! Verify Record owner wiring.
//! Wiring owner verify record.

#[path = "verify_record.rs"]
mod implementation;

pub(crate) use implementation::verify_source_checksum;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
