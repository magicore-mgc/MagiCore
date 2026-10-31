// Project context wiring — nối nhận diện và chính sách project Web.
// Keep detection, compatibility, and install setup in distinct owners.
use super::*;
#[path = "adapter_selection/mod.rs"]
pub(in crate::commands::core::web::implementation) mod adapter_selection;
#[path = "compatibility/mod.rs"]
mod compatibility;
#[path = "native_install_environment/mod.rs"]
pub(in crate::commands::core::web::implementation) mod native_install_environment;
#[path = "project_root/mod.rs"]
pub(in crate::commands::core::web::implementation) mod project_root;
pub use compatibility::check_dev_freshness;
pub(super) use compatibility::compat_install_target;
pub(in crate::commands::core::web::implementation) use compatibility::is_startable_server_half;
#[cfg(test)]
pub(in crate::commands::core::web::implementation) use compatibility::{
    compat_gate_decision, non_native_member_ecosystem,
};
