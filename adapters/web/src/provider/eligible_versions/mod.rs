//! eligible_versions owner wiring.
//! Wiring owner lọc candidate version theo age gate.
mod eligible_versions;
pub use eligible_versions::eligible_versions;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
