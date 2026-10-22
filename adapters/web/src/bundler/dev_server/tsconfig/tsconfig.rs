//! Fail-closed project tsconfig state and content digest.
//! Trạng thái tsconfig project fail-closed và digest nội dung.

use super::error::TsconfigDigestError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::bundler::dev_server::implementation) enum TsconfigState {
    Absent,
    Present(String),
}

impl TsconfigState {
    /// Cache-key fragment — stable, JSON-safe.
    /// (Mảnh key cache — ổn định, an toàn JSON.)
    pub(in crate::bundler::dev_server::implementation) fn key_fragment(&self) -> String {
        match self {
            TsconfigState::Absent => "absent".to_string(),
            TsconfigState::Present(digest) => format!("present:{digest}"),
        }
    }
}

/// Content digest of the project's tsconfig.json — recomputed per
/// request (one stat + one read of a small config file; the v2 identity
/// memo (mtime+len) is GONE because it could serve a stale digest on a
/// same-length rewrite with a restored mtime — the audit's standing
/// finding. A tsconfig is tiny; hashing it per request is cheap and
/// always correct).
///
/// Fail-closed semantics (Gate 11-C, vòng-11 audit — replacing the
/// poison-nonce scheme the round-10 report claimed as a fix):
///   - NotFound → `TsconfigState::Absent` — compiled with esbuild's
///     defaults, its own cache namespace (never collides with a digest);
///   - successful read → `TsconfigState::Present(blake3-of-bytes)`;
///   - ANY other error (permission denied, I/O, ELOOP) → `Err` — the
///     dev server answers 500 with an actionable message. The compiler
///     must NEVER compile under unknown semantics: a poison nonce
///     changed the key per call but still served a compile made with
///     WRONG (default) tsconfig options — stale-poison was fixed, wrong
///     semantics was not.
///
/// Digest nội dung tsconfig.json của project — tính lại mỗi request
/// (một stat + một đọc file config nhỏ; memo identity v2 (mtime+len) ĐÃ
/// BỎ vì nó có thể phục vụ digest stale khi ghi lại cùng độ dài và phục
/// hồi mtime — finding còn treo của audit. Tsconfig rất nhỏ; hash mỗi
/// request rẻ và luôn đúng).
///
/// Ngữ nghĩa fail-closed (thay scheme poison-nonce mà báo cáo vòng-10
/// gọi là bản sửa):
///   - NotFound → `TsconfigState::Absent` — biên dịch với default của
///     esbuild, namespace cache riêng (không bao giờ đụng digest);
///   - đọc thành công → `TsconfigState::Present(blake3-của-bytes)`;
///   - MỌI lỗi khác (từ chối quyền, I/O, ELOOP) → `Err` — dev server trả
///     500 kèm thông điệp xử lý được. Compiler KHÔNG BAO GIỜ được biên
///     dịch dưới semantics không xác định: poison nonce đổi key mỗi lần
///     gọi nhưng vẫn phục vụ một bản compile dùng tsconfig SAI (default)
///     — stale-poison được sửa, semantics sai thì không.)
pub(in crate::bundler::dev_server::implementation) fn tsconfig_digest(
    root: &std::path::Path,
) -> std::result::Result<TsconfigState, TsconfigDigestError> {
    let path = root.join("tsconfig.json");
    // Metadata FIRST, and only NotFound means absent — a PermissionDenied
    // or ELOOP here used to fold into "absent" (compiled with the WRONG
    // semantics while the file existed). Everything else is fatal.
    // (Metadata TRƯỚC, chỉ NotFound là vắng — PermissionDenied hay ELOOP
    // ở đây từng bị gộp vào "absent" (biên dịch với semantics SAI trong
    // khi file tồn tại). Mọi lỗi khác là fatal.)
    match std::fs::metadata(&path) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(TsconfigState::Absent);
        }
        Err(e) => {
            return Err(TsconfigDigestError::CannotAccess {
                path: path.clone(),
                source: e,
            });
        }
    }
    // Read the CONTENT — the identity cache (mtime+len) only avoids
    // rehashing on the hit path; a read error after a successful stat is
    // fatal (not absent, not poison — the request fails with 500).
    // (Đọc NỘI DUNG — cache identity (mtime+len) chỉ tránh rehash trên
    // đường hit; lỗi đọc sau khi stat thành công là fatal (không phải
    // absent, không phải poison — request fail với 500).)
    let bytes = std::fs::read(&path).map_err(|source| TsconfigDigestError::CannotRead {
        path: path.clone(),
        source,
    })?;
    let digest = blake3::Hasher::new()
        .update(&bytes)
        .finalize()
        .to_hex()
        .to_string();
    Ok(TsconfigState::Present(digest))
}
