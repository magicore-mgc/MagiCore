//! Validate safe Python site-directory names and runtime-compatible wheels.
//! Xác thực tên thư mục site an toàn và wheel tương thích runtime Python.

use mgc_resolver::protocols::PypiProtocol;
use mgc_types::ResolvedGraph;

use super::error::PythonRuntimeCompatibilityError;

pub(in crate::install) fn python_site_dirname(
    name: &str,
    version: &str,
) -> Result<String, PythonRuntimeCompatibilityError> {
    PypiProtocol::importable_site_dirname(name, version)
        .map_err(PythonRuntimeCompatibilityError::Resolver)
}

pub(in crate::install) fn validate_python_importable_graph(
    graph: &ResolvedGraph,
) -> Result<(), PythonRuntimeCompatibilityError> {
    for package in &graph.packages {
        if !PypiProtocol::is_importable_runtime_wheel(&package.tarball_url)
            .map_err(PythonRuntimeCompatibilityError::Resolver)?
        {
            let filename = PypiProtocol::artifact_filename_for_url(&package.tarball_url)
                .map_err(PythonRuntimeCompatibilityError::Resolver)?;
            let guidance = if filename.ends_with(".whl") {
                "compiled wheels need a host-compatible Python ABI; set MGC_PYTHON_VERSION to the exact Python minor used by `mgc run`/`mgc dev`"
            } else {
                "source distributions are not built by the native Python installer; use a compatible wheel"
            };
            return Err(PythonRuntimeCompatibilityError::ArtifactNotImportable {
                name: package.id.name_str().to_string(),
                version: package.id.version().to_string(),
                filename,
                guidance,
            });
        }
    }
    Ok(())
}
