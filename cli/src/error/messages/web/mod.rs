//! Web package, install, and dev diagnostics.
//! Lỗi package, install và dev của Web.

mod error;

pub use error::{
    bad_npm_response, install_slot_failed, install_task_failed, invalid_framework_spec,
    multiple_frameworks, network_error_fetching, no_framework_specified, no_primary_package,
    no_version_field, npm_registry_status, pinia_requires_vue, pm_not_supported, topo_order_failed,
    ts_js_exclusive, web_backend_flag_unsupported, web_dev_interrupted, web_dev_target_cancelled,
    web_empty_dev_script, web_forbidden_pm, web_generator_output_invalid,
    web_generator_output_unsafe, web_missing_executable, web_missing_package_json,
    web_no_dev_entrypoint, web_no_dev_target, web_no_install_flow, web_unsupported_dev_script,
    web_workspace_dep_missing,
};
