//! Package Operations owner wiring.
//! Wiring cho owner package operations.

#[path = "package_operations.rs"]
mod package_operations;
pub use package_operations::{
    adapter_package_list_rejects_template_inventory, add_fails_for_hardware_adapter,
    remove_fails_for_hardware_adapter,
};
