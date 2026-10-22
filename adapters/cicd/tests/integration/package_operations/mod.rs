//! Package Operations owner wiring — wiring cho owner package operations.

#[path = "package_operations.rs"]
mod package_operations;
pub use package_operations::{
    add_fails_closed_with_error_mentioning_deploy,
    install_fails_closed_because_no_package_semantics,
    list_fails_closed_because_pipeline_files_are_not_packages, remove_fails_closed,
    update_fails_closed,
};
