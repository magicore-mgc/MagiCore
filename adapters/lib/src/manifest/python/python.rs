//! Python manifest compatibility.
//! Tương thích manifest Python.

use mgc_types::{Manifest, MgResult};
use std::path::Path;

// COMPAT facade (S09a/P03) — delegates to mgc_python_service::manifest.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: adapter.rs python_native_owned + require_python_native_at,
//   language/dependency_format/dependency_format.rs dependency_manifest_format, install/mod.rs
//   native_python_path_entries_from_store (ownership guard), lib.rs re-export
//   (ai uses_native_python_lane), lib manifest tests.
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub fn supports_native_python_project(root: &Path) -> bool {
    mgc_python_service::manifest::supports_native_python_project(root)
}

// COMPAT facade (S09a/P03) — delegates to mgc_python_service::manifest.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: adapter.rs parse_manifest (Python arm), install/mod.rs
//   native_python_path_entries_from_store (manifest guard), lib manifest tests.
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(crate) fn parse_pyproject_manifest(root: &Path) -> MgResult<Manifest> {
    mgc_python_service::manifest::parse_pyproject_manifest(root)
}

// COMPAT facade (S09a/P03) — delegates to mgc_python_service::manifest.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: adapter.rs write_manifest (Python arm), lib manifest tests.
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(crate) fn write_pyproject_manifest(root: &Path, manifest: &Manifest) -> MgResult<()> {
    mgc_python_service::manifest::write_pyproject_manifest(root, manifest)
}
