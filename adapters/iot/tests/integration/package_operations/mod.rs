//! Package Operations owner wiring — wiring cho owner package operations.

#[path = "package_operations.rs"]
mod package_operations;
pub use package_operations::{
    platformio_adapter_install_fails_closed_without_spawning_pio,
    platformio_add_fails_closed_outside_explicit_cli_compatibility,
    platformio_list_does_not_report_declared_ranges_as_installed_versions,
    zephyr_add_fails_closed_without_native_dependency_support,
    zephyr_list_fails_closed_instead_of_returning_an_empty_native_list,
};
