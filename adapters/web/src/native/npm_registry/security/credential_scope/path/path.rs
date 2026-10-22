//! security credential scope path.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::registry_origin_matches;

// Match origin and a path segment boundary, refusing ambiguous encoded traversal.
// Giữ hỗ trợ @scope%2fpkg; dot segment hoặc backslash không được cấp token.
pub(in crate::native::npm_registry) fn credential_path_matches(scope: &str, target: &str) -> bool {
    if !registry_origin_matches(scope, target) {
        return false;
    }
    let (Ok(scope), Ok(target)) = (reqwest::Url::parse(scope), reqwest::Url::parse(target)) else {
        return false;
    };
    if !credential_path_is_unambiguous(scope.path())
        || !credential_path_is_unambiguous(target.path())
    {
        return false;
    }
    let prefix = scope.path().trim_end_matches('/');
    target.path() == prefix || target.path().starts_with(&format!("{prefix}/"))
}

// Bounded structural decoding is a policy check, not a server normalization claim.
// Không khẳng định mô hình mọi server; mã hóa quá sâu hoặc lỗi thì không gửi token.
pub(in crate::native::npm_registry) fn credential_path_is_unambiguous(path: &str) -> bool {
    let mut bytes = path.as_bytes().to_vec();
    for _ in 0..MAX_CREDENTIAL_PATH_DECODE_LAYERS {
        if bytes.contains(&b'\\')
            || bytes
                .split(|byte| *byte == b'/')
                .any(|segment| segment == b"." || segment == b"..")
        {
            return false;
        }
        let mut decoded = Vec::with_capacity(bytes.len());
        let mut index = 0;
        while index < bytes.len() {
            if bytes[index] == b'%' {
                if index + 2 >= bytes.len() {
                    return false;
                }
                let (Some(high), Some(low)) = (
                    (bytes[index + 1] as char).to_digit(16),
                    (bytes[index + 2] as char).to_digit(16),
                ) else {
                    return false;
                };
                decoded.push((high * 16 + low) as u8);
                index += 3;
            } else {
                decoded.push(bytes[index]);
                index += 1;
            }
        }
        if decoded == bytes {
            return true;
        }
        bytes = decoded;
    }
    false
}

pub(in crate::native::npm_registry) const SCOPED_REGISTRY_TIMEOUT_SECONDS: u64 = 300;

const MAX_CREDENTIAL_PATH_DECODE_LAYERS: usize = 8;
