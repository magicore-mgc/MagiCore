//! Defines validated package names.
//! Định nghĩa tên package đã xác thực.

use crate::error::{MgError, MgResult};
use std::fmt;

const MAX_PACKAGE_NAME_LEN: usize = 214;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PackageName(String);

impl PackageName {
    pub fn new(name: impl Into<String>) -> MgResult<Self> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(MgError::InvalidPackageName(name));
        }
        if trimmed.len() > MAX_PACKAGE_NAME_LEN {
            return Err(MgError::InvalidPackageName(name));
        }
        if trimmed.contains('\0') || trimmed.contains('\n') || trimmed.contains('\r') {
            return Err(MgError::InvalidPackageName(name));
        }
        if trimmed.contains("..") || trimmed.starts_with('/') || trimmed.starts_with('.') {
            return Err(MgError::InvalidPackageName(name));
        }
        if trimmed.contains('\\') || trimmed.contains('~') {
            return Err(MgError::InvalidPackageName(name));
        }
        let slash_count = trimmed.chars().filter(|&c| c == '/').count();
        if trimmed.starts_with('@') {
            // npm-style scope: exactly ONE slash with a real scope+id.
            // Scope kiểu npm: đúng MỘT slash với scope+id thật.
            if slash_count != 1 {
                return Err(MgError::InvalidPackageName(name));
            }
            let parts: Vec<&str> = trimmed.splitn(2, '/').collect();
            if parts[0].len() < 2 || parts[1].is_empty() {
                return Err(MgError::InvalidPackageName(name));
            }
        } else if trimmed.ends_with('/') {
            return Err(MgError::InvalidPackageName(name));
        }
        // Multi-slash WITHOUT a scope is the repo-path form used by
        // non-npm ecosystems (OSV SwiftURL: github.com/owner/repo) —
        // a legitimate identifier, not a path (cache layers sanitize
        // before any path join; mgc-exec blocks traversal separately).
        // Nhiều slash KHÔNG scope là dạng đường dẫn repo của ecosystem
        // ngoài npm (OSV SwiftURL: github.com/owner/repo) — định danh
        // hợp lệ, không phải path (tầng cache sanitize trước khi ghép
        // path; mgc-exec chặn traversal riêng).
        if trimmed.chars().any(|c| {
            !c.is_ascii_alphanumeric()
                && !matches!(c, '@' | '/' | '-' | '_' | '.' | '!' | '~' | ':')
        }) {
            return Err(MgError::InvalidPackageName(name));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn unscoped(&self) -> &str {
        self.0.rsplit('/').next().unwrap_or(self.as_str())
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
