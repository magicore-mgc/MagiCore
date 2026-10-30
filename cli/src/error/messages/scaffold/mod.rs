//! Project scaffold validation diagnostics.
//! Lỗi kiểm tra scaffold project.

mod error;

pub use error::{
    dir_already_exists, invalid_project_name, invalid_project_name_retries_exhausted,
    package_json_root_object, unknown_framework_for_core, unsupported_monorepo_frontend,
    unsupported_scaffold_framework, web_framework_not_yet_scaffoldable,
};
