//! Dependency ownership gate facade — facade cổng ownership dependency.
//!
//! This module preserves the existing `commands::dep_gate::*` API while wiring focused owners.
//! Module này giữ nguyên API hiện có và nối các owner theo trách nhiệm.

mod cloud_project_gate;
mod context;
mod dep_flag;
mod dep_gate;
mod ecosystem;
mod error;
mod gate;
mod lib_project_context;
mod native_adapter_gate;
mod operation;
mod ownership;
mod split_languages;

pub use cloud_project_gate::gate_cloud_project;
pub use context::DepContext;
pub use dep_flag::from_dep_flag;
pub use dep_gate::owner_for;
pub use ecosystem::eco;
pub(crate) use error::DepGateError;
pub use gate::gate;
pub use lib_project_context::lib_project_context;
pub use native_adapter_gate::gate_native_adapter;
pub use operation::DepOp;
pub use ownership::DepOwner;
pub use split_languages::SPLIT_LANGUAGES;

#[cfg(test)]
#[path = "test_support/mod.rs"]
pub(crate) mod test_support;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
