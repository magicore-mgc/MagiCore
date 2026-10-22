//! provider owner wiring.
//! Wiring owner state và khởi tạo provider.
mod provider;
pub use provider::NpmDependencyProvider;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
