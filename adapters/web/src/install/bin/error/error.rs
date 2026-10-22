//! Preserve the originating MagiCore error while identifying the `.bin` owner.
//! Giữ nguyên MgError gốc và phân loại theo owner `.bin`.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::install::bin) enum BinError {
    #[error("{0}")]
    Directory(#[source] MgError),
    #[error("{0}")]
    Manifest(#[source] MgError),
    #[error("{0}")]
    Link(#[source] MgError),
    #[cfg(unix)]
    #[error("{0}")]
    Executable(#[source] MgError),
}

impl From<BinError> for MgError {
    fn from(error: BinError) -> Self {
        match error {
            BinError::Directory(source) | BinError::Manifest(source) | BinError::Link(source) => {
                source
            }
            #[cfg(unix)]
            BinError::Executable(source) => source,
        }
    }
}
