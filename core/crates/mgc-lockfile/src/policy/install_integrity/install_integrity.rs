//! Validate install SRI shape and normalize native SHA-256 text.
//! Kiểm tra dạng SRI install và chuẩn hóa SHA-256 native.

use crate::{LockfileError, LockfileResult};

/// SRI algorithms the web materializer enforces (must stay in sync with
/// `adapters/web/src/install/integrity/::verify_sri_integrity`).
/// Thuật toán SRI materializer web bắt buộc (phải đồng bộ với
/// `verify_sri_integrity`).
pub const INSTALL_SRI_ALGORITHMS: &[&str] = &["sha256", "sha512"];

/// Check an SRI string parses as `<algo>-<base64>` with a supported strong
/// algorithm and full-length digest — fail-closed on malformed bytes.
/// The materializer re-verifies bytes at install; this gate only rejects
/// values that could never verify (never fabricates a hash).
/// Kiểm tra chuỗi SRI đúng dạng với thuật toán mạnh được hỗ trợ —
/// fail-closed với yếu/không rõ/trống. Materializer verify lại byte khi
/// install; gate này chỉ loại giá trị không bao giờ verify được.
pub fn check_install_sri(sri: &str) -> LockfileResult<()> {
    let fail =
        |why: &str| LockfileError::VerificationFailed(format!("unusable SRI integrity: {why}"));
    let Some((algorithm, payload)) = sri.split_once('-') else {
        return Err(fail("missing `<algo>-<base64>` shape"));
    };
    if !INSTALL_SRI_ALGORITHMS.contains(&algorithm) {
        return Err(fail(&format!(
            "algorithm '{algorithm}' is not enforced by the installer (sha256/sha512 only)"
        )));
    }
    if payload.is_empty()
        || !payload
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=')
    {
        return Err(fail("empty or non-base64 payload"));
    }
    // These lengths are algorithm invariants, not tunable resource limits.
    // Digest phải đúng số byte của thuật toán; alphabet đúng chưa đủ để verify.
    let expected_length: usize = match algorithm {
        "sha256" => 32,
        "sha512" => 64,
        _ => return Err(fail("unsupported digest algorithm")),
    };
    if payload.len() != expected_length.div_ceil(3) * 4 {
        return Err(fail("payload has the wrong encoded digest length"));
    }
    use base64::Engine;
    let digest = base64::engine::general_purpose::STANDARD
        .decode(payload)
        .map_err(|_| fail("payload is not canonical base64"))?;
    if digest.len() != expected_length {
        return Err(fail(&format!(
            "{algorithm} digest must decode to {expected_length} bytes"
        )));
    }
    Ok(())
}

/// Decode the SHA-256 representation used by native protocol verifiers.
/// Non-SHA256 algorithms retain their protocol-specific markers; malformed
/// SHA-256 payloads remain nonempty so native digest comparison refuses them.
/// Đổi digest SHA-256 legacy hoặc SRI sang hex cho verifier native, không tính hash.
/// Dữ liệu sai không bị biến thành thiếu digest rồi rơi sang đường tin cậy khác.
pub fn native_sha256_hex(integrity: &str) -> String {
    use base64::Engine;
    let Some(raw) = integrity.strip_prefix("sha256-") else {
        return String::new();
    };
    if raw.is_empty() {
        return integrity.to_string();
    }
    if raw.len() == 64 && raw.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return raw.to_ascii_lowercase();
    }
    if raw.len() == 44
        && let Ok(digest) = base64::engine::general_purpose::STANDARD.decode(raw)
        && digest.len() == 32
    {
        return hex::encode(digest);
    }
    raw.to_string()
}
