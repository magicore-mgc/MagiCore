//! Core install and language setup diagnostics.
//! Lỗi cài core và thiết lập ngôn ngữ.

mod error;

pub use error::{
    add_app_usage, ai_no_lockfile, ai_project_not_detected, app_multi_platforms_skipped,
    app_not_available, app_project_not_detected, app_tool_failed,
    global_package_install_unsupported, install_dry_run_unsupported, manifest_hint,
    no_ai_framework, no_app_language, objc_dev_needs_xcode, python_path_not_unicode,
    python_runtime_failed, python3_failed, remove_ai_usage, remove_app_usage,
    xcode_project_missing, xcode_project_missing_short,
};
