//! AI core helper facade.
//! Facade helper cho AI core.

mod dev_runner;
mod error;
mod project_identity;
mod python_lifecycle_gate;
mod runtime_detection;

#[cfg(feature = "ai")]
pub use dev_runner::ai_dev;
#[cfg(feature = "ai")]
pub use project_identity::ai_project_root;
#[cfg(feature = "ai")]
pub use python_lifecycle_gate::require_native_ai_python;
