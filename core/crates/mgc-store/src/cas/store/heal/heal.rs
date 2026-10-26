//! Detect and quarantine a CAS blob whose bytes no longer match its address.
//! Phát hiện và cách ly blob CAS có byte không còn khớp địa chỉ.
//!
//! A content-addressed store proves integrity only if it checks on the way out.
//! Import hashes what it writes; export verifies the destination. Between those
//! two points a blob can be changed by a bit flip, a half-finished write from a
//! killed process, or an attacker with write access — and the address alone says
//! nothing, because the address is just a name.
//! Store content-addressed chỉ chứng minh toàn vẹn nếu kiểm trên đường ra. Import
//! hash thứ nó ghi; export verify đích. Giữa hai điểm đó blob có thể bị đổi do
//! bit flip, do ghi dở khi process bị kill, hoặc do kẻ tấn công có quyền ghi —
//! và địa chỉ một mình không nói gì, vì địa chỉ chỉ là cái tên.
//!
//! Healing here means: verify, and on mismatch move the blob aside instead of
//! leaving it to be served. The caller then re-fetches from the source it
//! recorded, so a corrupt cache costs a download rather than a wrong artefact.
//! Chữa ở đây nghĩa là: verify, và khi lệch thì chuyển blob sang chỗ khác thay
//! vì để nó được phục vụ. Caller sau đó tải lại từ nguồn đã ghi, nên cache hỏng
//! tốn một lần tải chứ không thành artefact sai.

use std::path::{Path, PathBuf};

use crate::cas::integrity::IntegrityHash;

/// What verification found for one blob.
/// Kết quả verify cho một blob.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BlobState {
    /// The blob is absent; the caller must fetch it.
    /// Blob không có; caller phải tải.
    Missing,
    /// The blob's bytes hash to its address.
    /// Byte của blob hash ra đúng địa chỉ.
    Intact,
    /// The blob was corrupt and has been moved out of the store.
    /// Blob hỏng và đã được chuyển khỏi store.
    Quarantined { moved_to: PathBuf },
}

/// Verify the blob for `hash` under `root`, quarantining it when corrupt.
///
/// Returns the state so the caller decides whether to re-fetch. A quarantine
/// failure is reported as an error: leaving a known-corrupt blob in place would
/// let it be served later.
/// Verify blob cho `hash` dưới `root`, cách ly khi hỏng. Trả trạng thái để
/// caller quyết định tải lại. Lỗi cách ly được báo thành lỗi: để blob đã biết
/// hỏng nằm lại sẽ khiến nó bị phục vụ sau này.
pub(crate) fn verify_or_quarantine(
    root: &Path,
    hash: &IntegrityHash,
) -> Result<BlobState, std::io::Error> {
    let path = hash.cas_path(root);
    if !path.is_file() {
        return Ok(BlobState::Missing);
    }

    let bytes = std::fs::read(&path)?;
    let recomputed = IntegrityHash::from_bytes(&bytes, hash.is_executable());
    if recomputed.as_hex() == hash.as_hex() {
        return Ok(BlobState::Intact);
    }

    // Move aside rather than delete: a corrupt blob is evidence, and an operator
    // may want to know which shard or disk produced it.
    // Chuyển sang chỗ khác thay vì xoá: blob hỏng là bằng chứng, người vận hành
    // có thể cần biết shard hay đĩa nào tạo ra nó.
    let quarantine_dir = root.join("quarantine");
    std::fs::create_dir_all(&quarantine_dir)?;
    let moved_to = quarantine_dir.join(format!("{}.corrupt", hash.as_hex()));
    std::fs::rename(&path, &moved_to)?;
    Ok(BlobState::Quarantined { moved_to })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
