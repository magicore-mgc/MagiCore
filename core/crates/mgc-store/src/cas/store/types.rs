//! CAS store state and public data types.
//! State và type dữ liệu public của CAS store.

use std::path::PathBuf;

/// Per-process memo of verified CAS blobs (P0-4 throughput fix, hardened
/// per adversarial audit vòng-3: (len, mtime) is NOT a security boundary —
/// an attacker with store write access can rewrite same-length content and
/// restore the mtime).
///
/// Hardened design:
/// - Key = the FULL CAS address (digest hex + executable bit) — the audit's
///   Bypass B (exec/non-exec collision) is structurally impossible.
/// - Fingerprint (Unix) = (device, inode, size, mtime, ctime). ctime is
///   updated by the kernel on EVERY metadata/content change and CANNOT be
///   restored by user code (utimensat only touches atime/mtime) — the
///   audit's Bypass A (same-length rewrite + mtime restore) is caught.
///   inode+device also catch file replacement by rename (new inode).
/// - Windows has no cheap stable file identity (NTFS file IDs can rotate
///   across reboot; BirthId APIs are niche) → the memo is DISABLED there:
///   every verify rehashes. Correctness before throughput, exactly as the
///   audit prescribes ("if there is no stable file identity: re-hash").
///
/// The FIRST touch of a blob still rehashes fully (fail-closed). The memo
/// only survives while the file identity stands — any mutation, replacement
/// or metadata-restore changes the fingerprint and forces a fresh rehash.
///
/// Memo theo-process của blob CAS đã verify (fix throughput P0-4, harden
/// theo adversarial audit vòng-3: (len, mtime) KHÔNG phải ranh giới bảo mật
/// — kẻ có quyền ghi store có thể ghi lại nội dung cùng độ dài rồi phục
/// hồi mtime).
///
/// Thiết kế đã harden:
/// - Key = địa chỉ CAS ĐẦY ĐỦ (hex digest + bit executable) — Bypass B của
///   audit (collision exec/non-exec) về cấu trúc là không thể.
/// - Fingerprint (Unix) = (device, inode, size, mtime, ctime). ctime do
///   kernel cập nhật MỌI thay đổi metadata/nội dung và KHÔNG thể phục hồi
///   bằng user code (utimensat chỉ chạm atime/mtime) — Bypass A của audit
///   (ghi đè cùng độ dài + phục hồi mtime) bị bắt. inode+device cũng bắt
///   file bị thay bằng rename (inode mới).
/// - Windows không có file identity ổn định rẻ (NTFS file ID có thể xoay
///   vòng sau reboot; API BirthId quá niche) → memo TẮT ở đó: mọi verify
///   rehash. Correctness trước throughput, đúng như audit chỉ dẫn ("không
///   có stable file identity: rehash lại").
///
/// Lần CHẠM ĐẦU blob vẫn rehash đầy đủ (fail-closed). Memo chỉ hiệu lực khi
/// file identity còn đứng — mọi sửa đổi, thay thế hay phục hồi metadata
/// đều đổi fingerprint và buộc rehash lại.
#[derive(Debug, Clone)]
pub struct ContentStore {
    pub(super) root: PathBuf,
    /// Bounded verified-blob memo (P1-2 audit vòng-4): the old unbounded
    /// HashMap grew with every blob ever touched — a long-lived dev server
    /// or registry leaked memory per digest. Now an LRU with a fixed
    /// capacity: the LEAST recently used verification is evicted first,
    /// worst case a re-verify (fail-closed rehash), never a wrong serve.
    ///
    /// Memo blob-đã-verify CÓ GIỚI HẠN (P1-2 audit vòng-4): HashMap cũ
    /// không giới hạn phình theo từng blob từng chạm — dev server /
    /// registry chạy lâu rò memory theo số digest. Giờ là LRU với capacity
    /// cố định: verification ít dùng nhất bị đẩy ra trước, tệ nhất là
    /// re-verify (rehash fail-closed), không bao giờ serve sai.
    #[cfg(unix)]
    pub(super) verified:
        std::sync::Arc<parking_lot::Mutex<lru::LruCache<VerifiedBlobKey, UnixFileIdentity>>>,
    /// Monotonic hit/miss/eviction counters for the memo (P1-2): a
    /// security-relevant cache must be measurable, not guessed.
    /// (Bộ đếm hit/miss/eviction monotonic của memo (P1-2): cache liên quan
    /// bảo mật phải đo được, không đoán mò.)
    #[cfg(unix)]
    pub(super) memo_counters: std::sync::Arc<AtomicMemoCounters>,
}

/// Default capacity of the bounded verified-blob memo (P1-2 audit vòng-4).
/// A compile-time constant, not a magic literal; large enough to keep hot
/// install/export flows memoized, small enough that a dev session cannot
/// leak unbounded memory.
///
/// Capacity mặc định của memo blob-đã-verify có giới hạn (P1-2 audit
/// vòng-4). Const tập trung, không phải literal rải rác; đủ lớn để giữ
/// memo cho luồng install/export nóng, đủ nhỏ để phiên dev không rò memory
/// không giới hạn.
pub const MEMO_CAPACITY: usize = 4096;

/// Compile-time proof the memo capacity is usable as `NonZeroUsize` — a
/// zero value fails the build here instead of panicking at runtime.
/// (Chứng minh compile-time capacity khác 0 — giá trị 0 rớt build.)
#[cfg(unix)]
pub(super) const MEMO_NONZERO: std::num::NonZeroUsize =
    match std::num::NonZeroUsize::new(MEMO_CAPACITY) {
        Some(capacity) => capacity,
        None => panic!("MEMO_CAPACITY must be nonzero"),
    };

/// Atomic hit/miss/eviction counters (P1-2) — lock-free, monotonic.
/// (Bộ đếm hit/miss/eviction atomic (P1-2) — không khóa, monotonic.)
#[cfg(unix)]
#[derive(Debug, Default)]
pub(super) struct AtomicMemoCounters {
    pub(super) hits: std::sync::atomic::AtomicU64,
    pub(super) misses: std::sync::atomic::AtomicU64,
    pub(super) evictions: std::sync::atomic::AtomicU64,
}

/// Snapshot of memo counters (P1-2) — what tests and future metrics read.
/// (Ảnh chụp bộ đếm memo (P1-2) — thứ test và metrics tương lai đọc.)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MemoStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

/// Full CAS address as the memo key — digest + executable (the CAS path is
/// derived from exactly these two, so two entries with the same key are the
/// same file; the audit's exec/non-exec collision cannot happen).
///
/// Địa chỉ CAS đầy đủ làm key memo — digest + executable (path CAS suy ra
/// từ đúng 2 giá trị này nên 2 entry cùng key là cùng file; collision
/// exec/non-exec của audit không thể xảy ra).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct VerifiedBlobKey {
    pub(super) digest: String,
    pub(super) executable: bool,
}

/// Platform file identity for the verified-blob memo.
/// Unix: (device, inode, size, mtime-nanos, ctime-nanos) — ctime cannot be
/// restored by user code, inode changes on replacement.
/// Windows: no identity — the memo is disabled (always rehash).
///
/// File identity theo platform cho memo blob-đã-verify.
/// Unix: (device, inode, size, mtime-nanos, ctime-nanos) — ctime không thể
/// phục hồi bằng user code, inode đổi khi file bị thay.
/// Windows: không có identity — memo tắt (luôn rehash).
/// Unix file identity: device + inode + size + mtime + ctime.
/// (Identity file Unix: device + inode + size + mtime + ctime.)
/// `None` from `file_identity()` = no stable identity (never memoize);
/// `Some(UnixFileIdentity)` is a complete kernel-maintained fingerprint:
/// ctime cannot be restored by user code (utimensat only touches
/// atime/mtime), inode changes on file replacement.
/// (`None` từ `file_identity()` = không có identity ổn định (không memo
/// hóa); `Some(UnixFileIdentity)` là fingerprint hoàn chỉnh do kernel giữ:
/// ctime không thể phục hồi bằng user code (utimensat chỉ chạm
/// atime/mtime), inode đổi khi file bị thay.
#[cfg(unix)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct UnixFileIdentity {
    pub(super) dev: u64,
    pub(super) ino: u64,
    pub(super) size: u64,
    pub(super) mtime_nanos: u64,
    pub(super) ctime_nanos: u64,
}

/// Stage export bytes: try platform COW clone first, plain copy as fallback.
/// COW clones are INDEPENDENT files (copy-on-write) — a later mutation of
/// the export rehydrates a private block, it never writes through into the
/// CAS blob (the P0-1 hardlink-poisoning property is preserved).
/// NOTE: clonefile(2) requires the destination to NOT exist, so no file is
/// pre-created before the clone attempt — the fallback copy creates its
/// own destination.
///
/// Staging bytes export: thử clone COW theo platform trước, copy thường dự
/// phòng. Clone COW là file ĐỘC LẬP (copy-on-write) — sau này sửa file
/// export sẽ rehydrate block riêng, không bao giờ ghi ngược vào blob CAS
/// (giữ nguyên tính chất chống đầu độc hardlink của P0-1). LƯU Ý:
/// clonefile(2) yêu cầu đích CHƯA tồn tại nên không tạo file trước attempt
/// clone — nhánh copy dự phòng tự tạo đích của nó.
/// How export staging produced the temp file — tests assert the ACTUAL
/// branch used (a fast path that silently never runs is a false-green
/// waiting to happen, per adversarial audit vòng-3).
///
/// Cách staging export đã tạo temp — test assert branch THẬT được dùng
/// (fast path im lặng không chạy là false-green chờ xảy ra, theo audit
/// vòng-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagingKind {
    /// Kernel copy-on-write clone (macOS clonefile / Linux FICLONE).
    /// (Clone copy-on-write cấp kernel: clonefile macOS / FICLONE Linux.)
    CowClone,
    /// Plain byte-copy fallback.
    /// (Fallback copy bytes thường.)
    PlainCopy,
}

impl StagingKind {
    /// Human-readable name (benchmark labels, tests).
    /// (Tên đọc được (nhãn benchmark, test).)
    pub fn label(self) -> &'static str {
        match self {
            Self::CowClone => "clonefile/FICLONE (COW)",
            Self::PlainCopy => "plain copy",
        }
    }
}
