//! Reads a v4 lane merge base without discarding legacy-row evidence.
//! Đọc base merge lane v4 mà không làm mất dấu row legacy.

use crate::{LockfileError, LockfileResult};

/// Read a v4 merge base for lane writes: `Some` document when the lock on
/// disk is already v4; `None` with `had_legacy_rows` telling whether
/// sibling pins were left behind by a non-v4 base (the caller warns loudly
/// instead of silently dropping them).
/// (Đọc base v4 để lane merge: `Some` khi lock đĩa đã v4; `None` kèm
/// `had_legacy_rows` cho biết có pin khác bị bỏ lại vì base không v4
/// (caller cảnh báo rõ thay vì lặng lẽ rớt).)
pub fn read_v4_merge_base(
    path: &std::path::Path,
) -> LockfileResult<(Option<crate::canonical::LockfileV4>, bool)> {
    let bytes = match crate::parser::read_lockfile_bytes(path) {
        Ok(bytes) => bytes,
        Err(LockfileError::IoError(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok((None, false));
        }
        Err(error) => return Err(error),
    };
    let text = std::str::from_utf8(&bytes).map_err(|error| {
        LockfileError::ParseError(format!("lockfile is not valid UTF-8: {error}"))
    })?;
    match crate::parser::parse_document(text)? {
        crate::parser::LockDocument::V4(doc) => Ok((Some(doc), false)),
        crate::parser::LockDocument::Legacy(lock) => Ok((None, !lock.packages.is_empty())),
    }
}
