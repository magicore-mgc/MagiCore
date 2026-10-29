//! Byte/string digest operations and compatibility method surface.
//! Thao tác digest byte/string và bề mặt method tương thích.
//! Thao tác digest byte/chuỗi và bề mặt method tương thích.

use crate::CryptoResult;
use crate::blake3_signer::types::Blake3Hash;

/// Incremental BLAKE3 hasher for multi-part trees (keg fingerprints).
/// Hasher BLAKE3 tăng dần cho cây nhiều phần (fingerprint keg).
///
/// Exposes only `update`/`hex` so callers outside this crate never depend on
/// the BLAKE3 implementation directly.
/// (Chỉ mở `update`/`hex` để caller ngoài crate không phụ thuộc trực tiếp
/// implementation BLAKE3.)
pub struct Blake3StreamingHasher {
    inner: blake3::Hasher,
}

impl Default for Blake3StreamingHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Blake3StreamingHasher {
    pub fn new() -> Self {
        Self {
            inner: blake3::Hasher::new(),
        }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.inner.update(bytes);
    }

    pub fn hex(&self) -> String {
        self.inner.finalize().to_hex().to_string()
    }
}

/// BLAKE3 digest operations.
/// Các thao tác tính digest BLAKE3.
pub struct Blake3Hasher;

impl Blake3Hasher {
    /// Hash bytes using the BLAKE3 implementation.
    /// Hash byte bằng triển khai BLAKE3.
    #[inline]
    pub fn hash_bytes(data: &[u8]) -> Blake3Hash {
        Blake3Hash(*blake3::hash(data).as_bytes())
    }

    /// Hash UTF-8 string bytes.
    /// Hash các byte của chuỗi UTF-8.
    pub fn hash_string(value: &str) -> Blake3Hash {
        Self::hash_bytes(value.as_bytes())
    }

    /// Stream a file to avoid loading the whole input into memory.
    /// Đọc streaming để không nạp toàn bộ file vào bộ nhớ.
    pub fn hash_file(path: &std::path::Path) -> CryptoResult<Blake3Hash> {
        crate::blake3_signer::file::stream::hash_file(path)
    }

    /// Compatibility alias retained for callers of the old mmap API.
    /// Giữ alias tương thích cho caller của API mmap cũ.
    /// This now uses bounded-memory streaming because concurrent file truncation
    /// can invalidate a mapped page and terminate the process with SIGBUS.
    /// Cách này dùng streaming giới hạn bộ nhớ vì truncate đồng thời có thể làm
    /// trang memory-map mất hiệu lực và kết thúc tiến trình bằng SIGBUS.
    pub fn hash_file_mmap(path: &std::path::Path) -> CryptoResult<Blake3Hash> {
        crate::blake3_signer::file::mmap::hash_file_mmap(path)
    }

    /// Hash a readable stream without buffering the whole payload (SEC-11).
    /// Hash một stream không nạp toàn bộ payload vào RAM (SEC-11).
    ///
    /// Callers that must not load a large or untrusted file (keg verification,
    /// CAS heal) stream through this instead of `std::fs::read`.
    /// (Caller không được nạp file lớn/không tin cậy — verify keg, heal CAS —
    /// dùng streaming thay vì `std::fs::read`.)
    pub fn hash_reader<R: std::io::Read>(mut reader: R) -> std::io::Result<String> {
        let mut hasher = blake3::Hasher::new();
        let mut buffer = vec![0u8; 64 * 1024];
        loop {
            let read = reader.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }
        Ok(hasher.finalize().to_hex().to_string())
    }

    /// Compare a byte digest with the expected digest.
    /// So sánh digest của dữ liệu với digest mong đợi.
    #[inline]
    pub fn verify(data: &[u8], expected: &Blake3Hash) -> bool {
        Self::hash_bytes(data) == *expected
    }
}
