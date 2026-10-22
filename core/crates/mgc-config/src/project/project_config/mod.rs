//! ProjectConfig schema and method owner wiring.
//! Wiring schema và các owner method ProjectConfig.

mod constructor;
mod project_config;
mod script_grammar;

pub use project_config::ProjectConfig;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
