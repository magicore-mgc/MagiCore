//! Typed failures for registry token persistence.
//! Lỗi typed khi lưu token registry.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::login) enum TokenStorageError {
    #[error("No home directory")]
    NoHomeDirectory,
    #[error(transparent)]
    CurrentDirectory(#[from] std::io::Error),
    #[error(transparent)]
    Persistence(anyhow::Error),
}

pub(in crate::commands::login) fn into_diagnostic(error: TokenStorageError) -> anyhow::Error {
    match error {
        TokenStorageError::NoHomeDirectory => crate::error::no_home_dir(),
        TokenStorageError::CurrentDirectory(source) => anyhow::Error::new(source),
        TokenStorageError::Persistence(source) => source,
    }
}
