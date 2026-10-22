//! Package Operations owner wiring — wiring cho owner package operations.

#[path = "package_operations.rs"]
mod package_operations;
pub use package_operations::{
    add_fails_closed_for_terraform, parse_manifest_uses_dir_name_for_terraform,
    pulumi_without_package_json_list_fails_closed, remove_fails_closed_for_terraform,
    terraform_list_fails_closed_instead_of_returning_an_empty_native_list,
    terraform_package_adapter_install_fails_closed_without_spawning_terraform,
    update_fails_closed_for_terraform,
};
