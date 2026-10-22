//! Path-redacted failures from validating esbuild input provenance.
//! Lỗi đã loại path khi kiểm nguồn gốc input esbuild.

#[derive(Debug, thiserror::Error)]
pub(in crate::bundler::dependency_cache) enum MetafileInputsError {
    #[error("esbuild input metadata is missing or invalid")]
    InvalidMetadata,
    #[error("an esbuild input could not be resolved safely")]
    UnresolvableInput,
    #[error("esbuild included an input outside installed dependency roots")]
    OutsideAllowedRoots,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
