//! Web package lifecycle API and owner wiring.
//! API lifecycle của package Web và wiring các owner chức năng.

mod environment;
mod error;
mod identity_guard;
mod lifecycle;
mod manifest;
mod script_execution;
mod types;

pub use lifecycle::LifecycleRunner;
pub(crate) use manifest::load_package_scripts;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
