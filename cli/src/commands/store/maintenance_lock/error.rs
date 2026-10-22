//! Typed failure when Web store maintenance cannot acquire the install lock.
//! Lỗi có type khi thao tác bảo trì Web store không lấy được install lock.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands) enum MaintenanceLockError {
    #[error(
        "acquire Web project maintenance lock at {locks_root} with key {project_key}: {source}"
    )]
    Acquire {
        locks_root: String,
        project_key: String,
        #[source]
        source: mgc_store::CasGenerationError,
    },
}
