//! App core identity and shared operation policy.
//! Identity và policy thao tác dùng chung của App core.

#[path = "error/mod.rs"]
mod error;
mod manifest_hint;
mod project_identity;
mod react_native_gate;

pub(crate) use error::{native_lifecycle_error, project_not_detected_error};
pub use manifest_hint::manifest_hint;
pub use project_identity::{language, project_root};
pub use react_native_gate::gate_react_native;
