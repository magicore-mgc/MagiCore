//! Presence-only guard for dependency managers not owned by the native lane.
//! Guard chỉ dò sự hiện diện của manager ngoài lane native.

use std::path::Path;

use super::error::ExternalPythonManifestError;

pub(in crate::install) fn external_python_manifest_name(
    project_root: &Path,
) -> Result<Option<String>, ExternalPythonManifestError> {
    let entries =
        std::fs::read_dir(project_root).map_err(ExternalPythonManifestError::ReadDirectory)?;
    for entry in entries {
        let entry = entry.map_err(ExternalPythonManifestError::ReadEntry)?;
        let Some(name) = entry.file_name().to_str().map(str::to_ascii_lowercase) else {
            continue;
        };
        if matches!(
            name.as_str(),
            "uv.lock"
                | "poetry.lock"
                | "pdm.lock"
                | "pipfile"
                | "pipfile.lock"
                | "pylock.toml"
                | "setup.py"
                | "setup.cfg"
                | "environment.yml"
                | "environment.yaml"
                | "conda-lock.yml"
                | "conda-lock.yaml"
                | "requirements.in"
                | "constraints.txt"
        ) || (name.starts_with("requirements") && name.ends_with(".txt"))
            || (name.starts_with("pylock.") && name.ends_with(".toml"))
            || (name.starts_with("constraints-") && name.ends_with(".txt"))
        {
            return Ok(Some(name));
        }
    }
    Ok(None)
}
