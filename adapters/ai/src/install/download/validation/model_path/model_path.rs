use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

pub(in crate::install::download) fn validate_model_relative_path(value: &str) -> MgResult<PathBuf> {
    use std::path::Component;

    if value.is_empty() || value.contains(['\\', '\0', ':']) {
        return Err(MgError::Other(format!(
            "invalid model artifact path '{value}'"
        )));
    }
    let path = Path::new(value);
    let components = path.components().collect::<Vec<_>>();
    if components.is_empty()
        || components
            .iter()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(MgError::Other(format!(
            "model artifact path must be relative and contain no traversal: '{value}'"
        )));
    }
    Ok(path.to_path_buf())
}
