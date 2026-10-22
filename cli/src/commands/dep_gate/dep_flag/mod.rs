//! Dependency compatibility flag wiring — wiring cờ compatibility dependency.
mod dep_flag;
pub use dep_flag::from_dep_flag;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
