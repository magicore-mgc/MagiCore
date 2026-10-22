//! Write Record owner wiring.
//! Wiring owner write record.

#[path = "write_record.rs"]
mod implementation;

pub(crate) use implementation::write_source_checksum;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
