// compatibility owner wiring — nối owner compatibility.
// Re-export only to the intended parent boundary.
use super::*;
#[path = "compatibility.rs"]
mod implementation;
pub use implementation::check_dev_freshness;
pub(in crate::commands::core::web::implementation) use implementation::compat_install_target;
#[cfg(test)]
pub(in crate::commands::core::web::implementation) use implementation::{
    compat_gate_decision, non_native_member_ecosystem,
};
