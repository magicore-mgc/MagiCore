use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

use serde::Serialize;

use super::error::AuditAppendError;

// Serialize audit appends from parallel executor tasks within this process.
// Tuần tự hóa append audit giữa các task executor chạy song song trong process.
static APPEND_LOCK: Mutex<()> = Mutex::new(());

pub(crate) fn append_serialized(path: &Path, value: &impl Serialize) -> anyhow::Result<()> {
    let _guard = APPEND_LOCK
        .lock()
        .map_err(|_| anyhow::Error::new(AuditAppendError::LockPoisoned))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut line = serde_json::to_vec(value)?;
    line.push(b'\n');
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(&line)?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
