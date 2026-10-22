//! Build and bundler diagnostics.
//! Lỗi build và bundler.

mod error;

pub use error::{
    app_build_config_invalid, app_build_config_read_failed, app_build_language_ambiguous,
    app_build_language_invalid, app_build_platforms_invalid, build_multi_platforms_incomplete,
    build_no_artifact, build_not_supported, build_output_path_unsafe, build_toolchain_missing,
    dotnet_build_failed, esbuild_failed, go_build_failed, go_build_freshness_unverified,
    java_build_backend_unavailable, join_paths, link_node_modules_failed, no_project_found_build,
    python_build_failed,
};
