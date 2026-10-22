#[path = "project.rs"]
mod implementation;

pub(in crate::commands::build::implementation) use implementation::{
    ensure_native_build_mode, find_root, project_manifest_present, should_use_legacy_rust_build,
};
