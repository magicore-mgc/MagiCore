//! Serializes and parses complete v4 lock documents, then validates revision rules.
//! Serialize và parse toàn bộ lock document v4 rồi kiểm tra revision.

use crate::canonical::{LockfileV4, validate_v4_revision};
/// Serialize the FULL v4 document (payload + hash + signature +
/// timestamps + workspace). Field ORDER here does not matter for the
/// digest — only `canonical_toml` output does. The document must parse
/// back into an identical `LockfileV4`.
/// (Serialize TOÀN BỘ tài liệu v4. Thứ tự field ở đây không ảnh hưởng
/// digest — chỉ output `canonical_toml` mới ảnh hưởng.)
pub fn write_v4_document(lock: &LockfileV4) -> crate::LockfileResult<String> {
    Ok(toml::to_string_pretty(lock)?)
}

/// Parse a full v4 document (any field order), rejecting non-v4 versions.
/// (Parse toàn bộ tài liệu v4, từ chối version khác.)
pub fn parse_v4_document(text: &str) -> crate::LockfileResult<LockfileV4> {
    let lock: LockfileV4 = toml::from_str(text)?;
    validate_v4_revision(&lock)?;
    Ok(lock)
}
