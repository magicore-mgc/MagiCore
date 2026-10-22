#[path = "manifest.rs"]
mod implementation;

pub(in crate::commands::build::implementation) use implementation::{
    find_local_csproj, has_java_build_manifest,
};
