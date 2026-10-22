//! Dependency esbuild operation wiring.
//! Wiring thao tác esbuild dependency.

mod build;
#[path = "metafile_inputs/mod.rs"]
mod metafile_inputs;

pub(super) use build::bundle_dependency;
pub(super) use metafile_inputs::MetafileInputsError;
