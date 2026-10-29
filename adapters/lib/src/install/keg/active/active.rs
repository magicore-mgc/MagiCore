//! Active-version pointer for a library keg store.
//! Con trỏ version active cho store keg của một library.
//!
//! The pointer lives in `opt/<library>/<ACTIVE_POINTER_FILE>` and records the
//! active version plus the version it replaced, so a rollback after a failed
//! switch never has to guess what was active before.
//! Con trỏ nằm ở `opt/<library>/<ACTIVE_POINTER_FILE>`, ghi version active và
//! version bị thay thế, nên rollback sau switch lỗi không phải đoán.
//!
//! The write goes through `mgc_platform::atomic_replace` and the in-memory
//! value is replaced only after the file is in place: a failed write leaves the
//! caller's view of the active version exactly as before (fail-closed).
//! Ghi qua `mgc_platform::atomic_replace`; giá trị trong bộ nhớ chỉ đổi sau khi
//! file đã vào chỗ, nên ghi lỗi không làm đổi version active mà caller thấy.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

use crate::install::keg::error::{KegError, KegResult};
use crate::install::keg::layout::{KegLayout, validate_library_name};

/// Pointer file name inside `opt/<library>/`.
/// Tên file con trỏ trong `opt/<library>/`.
pub const ACTIVE_POINTER_FILE: &str = ".mgc-active.json";

static NEXT_STAGING: AtomicU64 = AtomicU64::new(0);

/// Persisted active-version state.
/// Trạng thái version active được lưu.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveState {
    /// Currently active version.
    /// Version đang active.
    pub version: String,
    /// Version active before the last successful switch, if any.
    /// Version active trước lần switch thành công gần nhất (nếu có).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
}

impl ActiveState {
    /// State with only a version (no previous switch yet).
    /// Trạng thái chỉ có version (chưa switch lần nào).
    pub fn initial(version: &str) -> Self {
        Self {
            version: version.to_string(),
            previous: None,
        }
    }
}

/// Read the active-version pointer for one library.
/// Đọc con trỏ version active của một library.
pub fn read_active(layout: &KegLayout, name: &str) -> KegResult<Option<ActiveState>> {
    validate_library_name(name)?;
    let path = pointer_path(layout, name);
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(KegError::io("read active version pointer", &path, error)),
    };
    let state: ActiveState =
        serde_json::from_slice(&bytes).map_err(|error| KegError::InvalidActive {
            name: name.to_string(),
            path: path.display().to_string(),
            reason: format!("unparsable pointer: {error}"),
        })?;
    if state.version.is_empty() {
        return Err(KegError::InvalidActive {
            name: name.to_string(),
            path: path.display().to_string(),
            reason: "empty active version".to_string(),
        });
    }
    Ok(Some(state))
}

/// Persist the active-version pointer atomically.
/// Lưu con trỏ version active một cách atomic.
pub fn write_active(layout: &KegLayout, name: &str, state: &ActiveState) -> KegResult<()> {
    validate_library_name(name)?;
    let path = pointer_path(layout, name);
    let parent = path.parent().ok_or_else(|| KegError::UnsafePath {
        path: path.display().to_string(),
        reason: "active pointer has no parent directory",
    })?;
    std::fs::create_dir_all(parent)
        .map_err(|error| KegError::io("create active pointer directory", parent, error))?;
    let encoded = serde_json::to_vec_pretty(state)
        .map_err(|error| KegError::Layout(format!("encode active version pointer: {error}")))?;
    let sequence = NEXT_STAGING.fetch_add(1, Ordering::Relaxed);
    let staging = parent.join(format!(
        ".{ACTIVE_POINTER_FILE}.{}.{sequence}.tmp",
        std::process::id()
    ));
    std::fs::write(&staging, &encoded)
        .map_err(|error| KegError::io("write active pointer staging", &staging, error))?;
    let result = (|| -> KegResult<()> {
        mgc_platform::atomic_replace::atomic_replace_file(&staging, &path)
            .map_err(|error| KegError::io("replace active version pointer", &path, error))?;
        sync_dir(parent)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&staging);
    }
    result
}

/// Path of the active-version pointer for one library.
/// Đường dẫn con trỏ version active của một library.
pub fn pointer_path(layout: &KegLayout, name: &str) -> PathBuf {
    layout.opt_dir(name).join(ACTIVE_POINTER_FILE)
}

fn sync_dir(directory: &Path) -> KegResult<()> {
    #[cfg(unix)]
    {
        std::fs::File::open(directory)
            .and_then(|handle| handle.sync_all())
            .map_err(|error| KegError::io("sync active pointer directory", directory, error))
    }
    #[cfg(not(unix))]
    {
        let _ = directory;
        Ok(())
    }
}
