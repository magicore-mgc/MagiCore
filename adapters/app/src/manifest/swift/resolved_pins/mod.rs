//! Wires Package.resolved loading and dependency pin projection.
//! Nối thao tác đọc Package.resolved và chiếu pin dependency.

mod resolved_pins;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::resolved_pins::{pin_range_for, read_pins};
