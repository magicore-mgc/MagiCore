//! Cross-process file-lock handle for one project install.
//! Handle file lock liên tiến trình cho một lần install project.
use super::super::super::errors::CasGenerationError;

/// Project-level exclusive install lock (Gate 11-A item 7). File:
/// `<store>/locks/install-<digest>.lock` — digest = blake3 of the
/// canonical project root, so every spelling of one project maps to one
/// lock file. ACQUISITION CONTRACT (P3 comment fix, fresh-context
/// review 2026-09-15): this is a TRY-lock — a second concurrent install
/// FAILS FAST with a typed error (the old doc claimed "blocks (with
/// retry)", which the code never did; the honest contract is fail-fast,
/// matching production package managers' install-lock UX).
/// (Khóa install độc quyền mức project. File:
/// `<store>/locks/install-<digest>.lock` — digest = blake3 của project
/// root chuẩn hóa, mọi cách viết một project ánh xạ một file lock. HỢP
/// ĐỒNG LẤY KHÓA: đây là TRY-lock — install song song thứ hai FAIL NGAY
/// với lỗi có type (doc cũ claim "chặn (kèm retry)" trong khi code
/// chưa bao giờ làm vậy; hợp đồng trung thực là fail-fast, khớp UX
/// install-lock của package manager production.)
pub struct ProjectInstallLock {
    file: std::fs::File,
    path: std::path::PathBuf,
}

impl ProjectInstallLock {
    /// Lock file root — the store's locks/ directory (Layout::locks_dir).
    /// RULE §12 (no hardcode): the base honors MAGICORE_STORE_ROOT /
    /// default_store_root().
    /// (Root của file lock — thư mục locks/ của store (Layout::locks_dir).
    /// RULE §12: base tôn trọng MAGICORE_STORE_ROOT / default_store_root().)
    fn locks_root() -> std::path::PathBuf {
        crate::default_store_root().join("locks")
    }

    /// Acquire against the DEFAULT store root (production path).
    /// (Lấy khóa theo store root MẶC ĐỊNH (đường production).)
    pub(in crate::database::database::project_install_lock) fn acquire(
        project_root: &str,
    ) -> std::result::Result<Self, CasGenerationError> {
        Self::acquire_at(&Self::locks_root(), project_root)
    }

    /// Acquire against an explicit locks root — the seam that keeps
    /// tests hermetic (no global env mutation, no user-store touch) and
    /// lets embedders place the lock elsewhere.
    /// (Lấy khóa theo root locks tường minh — đường giữ test kín (không
    /// đổi env toàn cục, không chạm store user) và cho embedder đặt
    /// lock ở nơi khác.)
    pub fn acquire_at(
        root: &std::path::Path,
        project_root: &str,
    ) -> std::result::Result<Self, CasGenerationError> {
        Self::acquire_impl(root, project_root)
    }

    /// Path of the handoff file that records which pid already holds the lock.
    /// Đường dẫn file handoff ghi pid đang giữ lock.
    ///
    /// It lives next to the lock so a caller in another crate can ask "is this
    /// project already locked by a live process?" without an environment
    /// variable a user could set (N-01).
    /// (File nằm cạnh lock để caller ở crate khác hỏi "project này đã bị một
    /// process còn sống giữ chưa?" mà không cần biến môi trường người dùng đặt
    /// được — N-01.)
    pub fn handoff_path(locks_root: &std::path::Path, project_root: &str) -> std::path::PathBuf {
        let canonical = std::fs::canonicalize(project_root)
            .unwrap_or_else(|_| std::path::PathBuf::from(project_root));
        let digest = mgc_crypto::Blake3Hasher::hash_string(&canonical.to_string_lossy()).to_hex();
        locks_root.join(format!("{digest}.handoff"))
    }

    /// Whether the handoff names this very process (or a live holder).
    /// Handoff có ghi chính process này (hoặc một người giữ còn sống) không.
    pub fn held_by_live_process(locks_root: &std::path::Path, project_root: &str) -> Option<u32> {
        let path = Self::handoff_path(locks_root, project_root);
        let pid: u32 = std::fs::read_to_string(&path).ok()?.trim().parse().ok()?;
        if process_is_alive(pid) {
            Some(pid)
        } else {
            None
        }
    }

    /// Lock file this handle holds open (used by tests and diagnostics).
    /// File lock mà handle này đang giữ (test và chẩn đoán dùng).
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    fn acquire_impl(
        root: &std::path::Path,
        project_root: &str,
    ) -> std::result::Result<Self, CasGenerationError> {
        std::fs::create_dir_all(root).map_err(|e| {
            CasGenerationError::Io(format!("create locks dir '{}': {e}", root.display()))
        })?;
        // Digest via the shared crypto crate — mgc-store does not depend on
        // blake3 directly (dependency discipline, no duplicate hashers).
        // (Digest qua crate crypto dùng chung — mgc-store không phụ thuộc
        // blake3 trực tiếp (kỷ luật dependency, không nhân đôi hasher).)
        // SEC-07: the key must be the canonical project path. Hashing the raw
        // string let `P`, `P/`, `P/../P` and a symlinked alias of the same
        // project take different locks, so two installs could write one tree
        // at the same time.
        // (SEC-07: key phải là path canonical. Hash chuỗi thô khiến `P`, `P/`,
        // `P/../P` và alias symlink của cùng project nhận lock khác nhau, nên
        // hai lần cài có thể ghi cùng một cây cùng lúc.)
        let canonical = std::fs::canonicalize(project_root)
            .unwrap_or_else(|_| std::path::PathBuf::from(project_root));
        let digest = mgc_crypto::Blake3Hasher::hash_string(&canonical.to_string_lossy()).to_hex();
        let path = root.join(format!("install-{digest}.lock"));
        // SEC-08: the lock file must not be reachable through a planted
        // symlink in the store, so the open refuses links where the platform
        // can express that.
        // (SEC-08: file khoá không được đi qua symlink bị cài trong store, nên
        // lần mở từ chối link ở nền tảng diễn đạt được.)
        #[cfg(unix)]
        use std::os::unix::fs::OpenOptionsExt as _;
        let mut options = std::fs::OpenOptions::new();
        options.create(true).truncate(false).write(true);
        #[cfg(unix)]
        options.custom_flags(no_follow_flag());
        let file = options
            .open(&path)
            .map_err(|e| CasGenerationError::Io(format!("open lock '{}': {e}", path.display())))?;
        fs2::FileExt::try_lock_exclusive(&file).map_err(|e| {
            CasGenerationError::Io(format!(
                "another install holds the project lock \
                 ('{}' — concurrent installs of one project are serialized): {e}",
                path.display()
            ))
        })?;
        Ok(Self { file, path })
    }
}

/// Whether a pid names a live process.
/// Pid có trỏ tới process còn sống không.
fn process_is_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        // Signal 0 probes existence without delivering anything.
        // (Signal 0 chỉ kiểm sự tồn tại, không gửi gì.)
        if pid == 0 {
            return false;
        }
        // SAFETY: `kill(pid, 0)` only probes existence; it delivers no signal
        // and the pid is not dereferenced.
        // AN TOÀN: `kill(pid, 0)` chỉ kiểm tồn tại; không gửi signal và pid
        // không bị dereference.
        #[allow(unsafe_code)]
        let alive = unsafe { libc::kill(pid as libc::pid_t, 0) } == 0;
        // `EPERM` means the process exists but belongs to another user.
        // (`EPERM` nghĩa là process tồn tại nhưng thuộc user khác.)
        alive || std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM)
    }
    #[cfg(not(unix))]
    {
        // Windows has no cheap equivalent here; the handoff is only an
        // optimisation inside one CLI run, and a stale file falls back to
        // taking the lock normally.
        // (Windows không có cách tương đương rẻ; handoff chỉ là tối ưu trong
        // một lần chạy CLI, file cũ sẽ quay về việc lấy lock bình thường.)
        let _ = pid;
        true
    }
}

/// `O_NOFOLLOW` where the platform has it; zero elsewhere.
/// `O_NOFOLLOW` ở nền tảng có hỗ trợ; 0 ở nơi khác.
fn no_follow_flag() -> i32 {
    #[cfg(target_os = "linux")]
    {
        // O_NOFOLLOW
        0o400000
    }
    #[cfg(all(unix, not(target_os = "linux")))]
    {
        // O_NOFOLLOW on BSD/macOS
        0x0100
    }
    #[cfg(not(unix))]
    {
        0
    }
}

impl std::fmt::Debug for ProjectInstallLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectInstallLock")
            .field("path", &self.path)
            .finish()
    }
}

impl Drop for ProjectInstallLock {
    fn drop(&mut self) {
        fs2::FileExt::unlock(&self.file).ok();
        // The lock FILE stays — a zero-byte lock file is the standard
        // pattern (flock on the inode); removing it on drop re-opens the
        // rename race on POSIX.
        // (File lock GIỮ NGUYÊN — file lock 0-byte là pattern chuẩn (flock
        // trên inode); xóa nó lúc drop mở lại race rename trên POSIX.)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
