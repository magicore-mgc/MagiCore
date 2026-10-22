//! Shared private imports and Go module fixture for manifest tests.
//! Import chung private và fixture module Go cho test manifest.

pub(super) use crate::language::{LibLanguage, dependency_manifest_format};
pub(super) use crate::manifest::{
    parse_cargo_manifest, parse_go_mod_manifest, parse_pyproject_manifest,
    supports_native_python_project, write_cargo_manifest, write_pyproject_manifest,
};
pub(super) use mgc_types::capabilities::DependencyResolver;

pub(super) fn write_go_mod(dir: &std::path::Path, content: &str) {
    std::fs::write(dir.join("go.mod"), content).unwrap();
}
