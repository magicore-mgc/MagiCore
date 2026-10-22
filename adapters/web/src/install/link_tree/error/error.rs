//! Classify strict package-link failures and retain their underlying sources.
//! Phân loại lỗi liên kết package strict và giữ source gốc.

use mgc_types::MgError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::install::link_tree::implementation) enum LinkTreeOperation {
    InitializePool,
    CreatePackageLinkParent,
    SymlinkPackage,
    LinkPackageTree,
    CreateTreeTarget,
    WalkSourceEntry,
    ResolveRelativePath,
    CreateTreeDirectory,
    RemoveBeforeReflink,
    ReflinkFile,
    CreateBackingParent,
    RemoveBeforeCopy,
    CopyFile,
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub(in crate::install::link_tree::implementation) struct LinkTreeError {
    operation: LinkTreeOperation,
    message: String,
    #[source]
    source: anyhow::Error,
}

impl LinkTreeError {
    pub(in crate::install::link_tree::implementation) fn with_source<E>(
        operation: LinkTreeOperation,
        message: String,
        source: E,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            operation,
            message,
            source: anyhow::Error::new(source),
        }
    }

    #[cfg(test)]
    pub(super) fn operation_kind(&self) -> LinkTreeOperation {
        self.operation
    }
}

impl From<LinkTreeError> for MgError {
    fn from(error: LinkTreeError) -> Self {
        let _operation = error.operation;
        Self::Anyhow(anyhow::Error::new(error))
    }
}
