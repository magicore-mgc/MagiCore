//! Package Operations owner wiring — wiring cho owner package operations.

#[path = "package_operations.rs"]
mod package_operations;
pub use package_operations::{
    bevy_adapter_dependency_operations_fail_closed_without_spawning_cargo,
    bevy_list_does_not_report_manifest_ranges_as_installed_versions,
    godot_add_fails_closed_until_mgc_owns_dependency_resolution,
    godot_install_fails_closed_pointing_to_editor,
    godot_list_fails_closed_instead_of_returning_an_empty_native_list,
    unity_add_fails_closed_until_mgc_owns_upm_resolution,
};
