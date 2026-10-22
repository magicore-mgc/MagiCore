//! Project detection and core selection diagnostics.
//! Lỗi nhận diện project và chọn core.

mod error;

pub use error::{
    bare_core_not_detected, cannot_detect_project_type, core_not_in_build, cwd_deleted,
    detect_core_failed, mgc_toml_missing, no_framework_detected, no_mgc_project_found,
    no_mgc_project_root, no_registry_configured, pkg_not_found_local, project_root_missing,
    too_many_packages, tool_failed, unknown_core, unknown_ecosystem, workspace_failed,
};
