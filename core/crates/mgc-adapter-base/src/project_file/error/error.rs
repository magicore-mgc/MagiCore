//! Preserve the current project-file error representation.
//! Giữ cách biểu diễn lỗi project-file hiện tại.

use mgc_types::MgError;
use std::error::Error;
use std::fmt;

/// Error classification exposed through the project-file security boundary.
/// Phân loại lỗi public để caller biết đây là vi phạm policy hay lỗi filesystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectFileErrorKind {
    /// The path or file violates a project-file policy.
    /// Path hoặc file vi phạm policy project-file.
    PolicyViolation,
    /// An operating-system operation failed and retains its source error.
    /// Thao tác hệ điều hành thất bại và giữ source error gốc.
    FilesystemOperation,
}

/// Recovery guidance; this owner never retries a filesystem operation itself.
/// Gợi ý xử lý; owner này không tự retry thao tác filesystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectFileRecovery {
    /// Correct the supplied path or project input before retrying the request.
    /// Sửa path hoặc input project trước khi gửi lại request.
    CorrectInput,
    /// Inspect filesystem state or permissions; this owner does not retry.
    /// Kiểm tra filesystem hoặc permission; owner này không tự retry.
    InspectFilesystem,
}

/// Typed project-file error that retains the operating-system cause when present.
/// Lỗi project-file có type và giữ nguyên nguyên nhân từ hệ điều hành nếu có.
#[derive(Debug)]
pub struct ProjectFileError {
    kind: ProjectFileErrorKind,
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ProjectFileError {
    /// Return the error classification without parsing its display text.
    /// Lấy phân loại lỗi mà không cần phân tích chuỗi hiển thị.
    pub fn kind(&self) -> ProjectFileErrorKind {
        self.kind
    }

    /// Return the unchanged user-facing error message.
    /// Lấy thông điệp hiển thị được giữ nguyên.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the prescribed recovery step; no automatic retry is authorized.
    /// Lấy bước xử lý được khuyến nghị; không cho phép tự retry.
    pub fn recovery(&self) -> ProjectFileRecovery {
        match self.kind {
            ProjectFileErrorKind::PolicyViolation => ProjectFileRecovery::CorrectInput,
            ProjectFileErrorKind::FilesystemOperation => ProjectFileRecovery::InspectFilesystem,
        }
    }
}

impl fmt::Display for ProjectFileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ProjectFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}

pub(in crate::project_file) fn error(message: impl Into<String>) -> MgError {
    MgError::Anyhow(anyhow::Error::new(ProjectFileError {
        kind: ProjectFileErrorKind::PolicyViolation,
        message: message.into(),
        source: None,
    }))
}

pub(in crate::project_file) fn error_with_source<E, F>(source: E, message: F) -> MgError
where
    E: Error + Send + Sync + 'static,
    F: FnOnce(&E) -> String,
{
    let message = message(&source);
    MgError::Anyhow(anyhow::Error::new(ProjectFileError {
        kind: ProjectFileErrorKind::FilesystemOperation,
        message,
        source: Some(Box::new(source)),
    }))
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
