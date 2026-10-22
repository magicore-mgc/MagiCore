//! Shared runtime-launcher policy API.
//! API policy dùng chung cho launcher runtime.

mod argument_validation;
mod error;
mod launcher_policy;
mod runtime;

pub use launcher_policy::LauncherPolicy;
pub use runtime::Runtime;
