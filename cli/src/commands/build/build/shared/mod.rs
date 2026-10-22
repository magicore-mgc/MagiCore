#[path = "shared.rs"]
mod implementation;

pub(super) use implementation::{
    ensure_native_build_mode, find_root, go_offline_env, project_manifest_present,
    should_use_legacy_rust_build,
};
pub(crate) use implementation::{python_cmd, tool_unavailable};
