//! Validate reported bundle inputs before accepting the esbuild output.
//! Kiểm input được báo trước khi chấp nhận output esbuild.

use std::path::{Path, PathBuf};

use super::{document::validate_document, error::MetafileInputsError, input_path::validate_input};

/// Validate every reported input before output can enter the dependency cache.
/// Kiểm từng input được báo trước khi output có thể vào cache dependency.
pub(in crate::bundler::dependency_cache::build) fn validate_metafile_inputs(
    metafile: &str,
    working_dir: &Path,
    allowed_roots: &[PathBuf],
) -> Result<(), MetafileInputsError> {
    validate_document(metafile, |input| {
        validate_input(input, working_dir, allowed_roots)
    })
}
