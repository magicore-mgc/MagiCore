//! Typed errors for project source-file resolution.
//! Lỗi có kiểu khi resolve file source trong project.

#[derive(Debug, thiserror::Error)]
pub(in crate::bundler::dev_server::implementation) enum SourceFileResolutionError {
    #[error("project file path is private or outside the project root")]
    Forbidden,
    #[error("failed to resolve a project file")]
    Io(#[source] std::io::Error),
}
