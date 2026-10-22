//! Typed errors at the restore command boundary.
//! Lỗi có type tại biên lệnh restore.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(super) enum RestoreError {
    #[error("validate restore backup: {source}")]
    ValidateBackup {
        #[source]
        source: anyhow::Error,
    },
    #[error("acquire project Web-store lock before restore: {source}")]
    AcquireProjectLock {
        #[source]
        source: anyhow::Error,
    },
    #[error("resolve project Web-store database path: {source}")]
    ResolveDatabasePath {
        #[source]
        source: anyhow::Error,
    },
    #[error("inspect current project Web-store state: {source}")]
    InspectProjectState {
        #[source]
        source: anyhow::Error,
    },
    #[error("prepare pre-restore safety backup: {source}")]
    PrepareSafetyBackup {
        #[source]
        source: anyhow::Error,
    },
    #[error(
        "safety-backup destination changed during preparation: expected={expected}, actual={actual}"
    )]
    SafetyBackupDestinationChanged { expected: PathBuf, actual: PathBuf },
    #[error(
        "safety backup overlaps the selected restore source: source={backup_root}, safety={safety}"
    )]
    BackupOverlap { backup_root: String, safety: String },
    #[error("preserve current project state before restore at {safety_backup}: {source}")]
    PreserveCurrentState {
        safety_backup: PathBuf,
        #[source]
        source: anyhow::Error,
    },
    #[error(
        "restore project database failed; pre-restore state is preserved at {safety_backup}: {source}"
    )]
    RestoreDatabase {
        safety_backup: PathBuf,
        #[source]
        source: anyhow::Error,
    },
    #[error(
        "restore project virtual store failed; pre-restore state is preserved at {safety_backup}: {source}"
    )]
    RestoreVirtualStore {
        safety_backup: PathBuf,
        #[source]
        source: anyhow::Error,
    },
}
