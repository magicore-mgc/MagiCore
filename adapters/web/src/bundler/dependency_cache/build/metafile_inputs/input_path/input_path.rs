//! Resolve and contain one metafile input path.
//! Resolve và containment một path input metafile.

use std::path::{Path, PathBuf};

use super::super::error::MetafileInputsError;

/// Require the resolved input to be a file inside an allowed package root.
/// Yêu cầu input resolve thành file nằm trong root package được phép.
pub(in crate::bundler::dependency_cache::build::metafile_inputs) fn validate_input(
    input: &str,
    working_dir: &Path,
    allowed_roots: &[PathBuf],
) -> Result<(), MetafileInputsError> {
    let path = Path::new(input);
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        working_dir.join(path)
    };
    let canonical =
        std::fs::canonicalize(candidate).map_err(|_| MetafileInputsError::UnresolvableInput)?;
    if !std::fs::metadata(&canonical)
        .map_err(|_| MetafileInputsError::UnresolvableInput)?
        .is_file()
    {
        return Err(MetafileInputsError::UnresolvableInput);
    }
    if !allowed_roots.iter().any(|root| canonical.starts_with(root)) {
        return Err(MetafileInputsError::OutsideAllowedRoots);
    }

    Ok(())
}
