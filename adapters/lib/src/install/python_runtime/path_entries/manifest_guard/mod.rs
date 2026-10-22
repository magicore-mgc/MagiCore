//! Wires manifest safety and lock-pin checks.
//! Wiring kiểm tra an toàn manifest và pin trong lock.

mod error;
mod manifest_guard;

pub(super) use manifest_guard::{load_python_manifest, validate_manifest_pins};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
