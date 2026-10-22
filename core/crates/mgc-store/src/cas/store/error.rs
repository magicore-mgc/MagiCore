//! CAS error contract — preserves typed failures and source conversions.
//! Hợp đồng lỗi CAS — giữ nguyên lỗi typed và chuyển đổi source.

use std::path::PathBuf;

#[derive(Debug)]
pub enum StoreError {
    Io {
        path: PathBuf,
        msg: String,
    },
    NotFound(String),
    HashMismatch {
        expected: String,
        actual: String,
    },
    InvalidHash(String),
    /// Two writers published DIVERGENT outputs for the same compilation key
    /// (P0-2 audit vòng-4): the valid winner stays in place, the loser gets
    /// a typed conflict — never a faked Ok, never a silent overwrite.
    ///
    /// (Hai writer publish output PHÂN KỲ cho cùng compilation key (P0-2
    /// audit vòng-4): winner hợp lệ giữ nguyên vị trí, bên thua nhận
    /// conflict typed — không giả Ok, không đè im lặng.)
    CacheConflict {
        path: PathBuf,
        expected: String,
        winner: String,
    },
    /// A compiled-cache entry failed self-validation (P0-3 audit vòng-4)
    /// — key/digest verification failed (poison/tamper) or the bytes are
    /// torn/unparseable. The corrupt bytes are quarantined for forensics;
    /// the caller must fail closed (dev server: treat as a miss, recompile).
    ///
    /// (Entry compiled-cache rơi khỏi bước tự xác thực (P0-3 audit vòng-4)
    /// — verify key/digest fail (đầu độc/gian lận) hoặc bytes đứt/không
    /// parse được. Bytes hỏng bị cách ly để điều tra; caller phải fail cứng
    /// (dev server: coi là miss, biên dịch lại).)
    CacheCorrupt {
        path: PathBuf,
        detail: String,
    },
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, msg } => write!(f, "IO error at {}: {msg}", path.display()),
            Self::NotFound(hash) => write!(f, "content not found: {hash}"),
            Self::HashMismatch { expected, actual } => {
                write!(f, "hash mismatch: expected {expected}, got {actual}")
            }
            Self::InvalidHash(input) => write!(f, "invalid blake3 digest: {input}"),
            Self::CacheConflict {
                path,
                expected,
                winner,
            } => write!(
                f,
                "compiled-cache conflict at {}: expected output digest {expected}, but a divergent winner holds {winner} — nondeterministic compilation or tampering",
                path.display()
            ),
            Self::CacheCorrupt { path, detail } => write!(
                f,
                "compiled-cache entry failed self-validation at {}: {detail}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        Self::Io {
            path: PathBuf::new(),
            msg: e.to_string(),
        }
    }
}

impl From<super::super::integrity::InvalidHashError> for StoreError {
    fn from(e: super::super::integrity::InvalidHashError) -> Self {
        Self::InvalidHash(e.input)
    }
}
