//! Virtual-store payload measurement owner wiring.
//! Wiring owner đo payload virtual store.

mod measurement;
pub(super) use measurement::logical_payload_bytes;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
