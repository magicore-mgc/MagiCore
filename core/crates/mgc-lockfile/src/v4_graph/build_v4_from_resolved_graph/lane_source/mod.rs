//! lane_source owner wiring / wiring owner lane_source.

#[path = "lane_source.rs"]
mod implementation;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use self::implementation::{lane_source, source_id_for};
