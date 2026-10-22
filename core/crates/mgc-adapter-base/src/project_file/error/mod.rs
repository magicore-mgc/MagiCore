//! Local error conversion used by the project-file security owners.
//! Chuyển lỗi nội bộ theo hợp đồng MgError hiện hành.

mod error;
pub use self::error::{ProjectFileError, ProjectFileErrorKind, ProjectFileRecovery};
pub(super) use self::error::{error, error_with_source};
