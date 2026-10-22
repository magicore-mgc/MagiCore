//! release_age owner wiring.
//! Wiring owner policy tuổi release.
mod release_age;
pub use release_age::AgePolicy;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
