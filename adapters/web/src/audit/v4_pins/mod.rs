//! v4 pin selection owner facade. / Facade owner chọn pin v4.

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
mod v4_pins;

pub(crate) use v4_pins::v4_pins_for_audit;
