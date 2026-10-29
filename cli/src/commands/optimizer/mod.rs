//! `optimizer/mod.rs` — MagiCore Core-Neutral Optimizer Engine facade (runtime detection + adapter pattern).
//! Facade engine optimizer độc lập core: phát hiện runtime + dispatch adapter.

pub mod adapters;
pub mod detect;
pub mod env_loader;
pub mod generators;
pub mod runtime_detect;

mod optimize;
mod run;

pub use optimize::optimize_project;
pub use run::run;

#[cfg(test)]
#[path = "test/optimizer.rs"]
mod tests;

#[cfg(test)]
#[path = "test/adapters_test.rs"]
mod adapters_tests;

#[cfg(test)]
#[path = "test/core_coverage/mod.rs"]
mod core_coverage_tests;
