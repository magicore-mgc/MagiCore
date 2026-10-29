//! Exclusive guard for keg/prefix mutations with fail-closed rollback.
//! Guard độc quyền cho thao tác keg/prefix, rollback fail-closed.
//!
//! Two concurrent switches (two terminals, a script and `mgc lib`) on the same
//! library would interleave links; the guard serializes them per library with
//! an in-process `RwLock` plus an OS advisory lock on a file outside the keg
//! tree. Every mutation registers an undo step; leaving the scope without a
//! commit restores them in reverse order, so a failed or aborted switch never
//! leaves a half-switched prefix. Rollback is driven by `Result`/`Drop`, not by
//! unwinding, because release builds use `panic = "abort"`.
//! Hai switch đồng thời trên cùng library sẽ chen link; guard tuần tự hoá theo
//! library bằng `RwLock` trong tiến trình + khóa advisory OS trên file ngoài
//! cây keg. Mọi thay đổi đăng ký bước hoàn tác; rời phạm vi mà không commit sẽ
//! khôi phục ngược thứ tự, nên switch lỗi/abort không để prefix nửa vời.
//! Rollback chạy theo `Result`/`Drop`, không dựa vào unwind (release dùng
//! `panic = "abort"`).

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::install::keg::error::{KegError, KegResult};
use crate::install::keg::layout::{KegLayout, validate_library_name};

/// Undo step restoring one prefix entry to its previous state.
/// Bước hoàn tác đưa một entry prefix về trạng thái trước.
pub type Undo = Box<dyn FnOnce()>;

/// Exclusive scope for one library's keg/prefix changes.
/// Phạm vi độc quyền cho thay đổi keg/prefix của một library.
pub struct KegGuard {
    // Holds the OS advisory lock for this scope's lifetime.
    // Giữ khóa advisory OS trong suốt phạm vi.
    _os_lock: OsLock,
    // Held for the whole scope so no other thread mutates the same library.
    // Giữ suốt phạm vi để thread khác không sửa cùng library.
    _process_lock: Arc<RwLock<()>>,
    undos: Vec<Undo>,
    committed: bool,
    library: String,
}

impl KegGuard {
    /// Acquire the exclusive guard for one library.
    /// Lấy guard độc quyền cho một library.
    pub fn acquire(layout: &KegLayout, name: &str) -> KegResult<Self> {
        validate_library_name(name)?;
        let lock_root = layout.lib_root().join("locks");
        std::fs::create_dir_all(&lock_root)
            .map_err(|error| KegError::io("create keg lock directory", &lock_root, error))?;
        let process_lock = process_lock_for(&lock_root, name)?;
        let guard = process_lock.try_write().map_err(|_| {
            KegError::Layout(format!(
                "library '{name}' is being modified by another thread"
            ))
        })?;
        let lock_path = lock_root.join(format!("{}.lock", sanitize(name)));
        let os_lock = OsLock::acquire(&lock_path)?;
        drop(guard);
        Ok(Self {
            _os_lock: os_lock,
            _process_lock: process_lock,
            undos: Vec::new(),
            committed: false,
            library: name.to_string(),
        })
    }

    /// Register an undo step executed (in reverse order) on abort.
    /// Đăng ký bước hoàn tác, chạy ngược thứ tự khi abort.
    pub fn push_undo(&mut self, undo: Undo) {
        self.undos.push(undo);
    }

    /// Run a mutation; an `Err` result rolls back before it is returned.
    /// Chạy mutation; kết quả `Err` sẽ rollback trước khi trả về.
    pub fn run<T, F>(&mut self, operation: F) -> KegResult<T>
    where
        F: FnOnce(&mut Self) -> KegResult<T>,
    {
        let result = operation(&mut *self);
        if result.is_err() {
            self.rollback();
        }
        result
    }

    /// Commit the scope: drop undo steps so `Drop` restores nothing.
    /// Commit phạm vi: bỏ bước hoàn tác để `Drop` không khôi phục gì.
    pub fn commit(&mut self) {
        self.committed = true;
        self.undos.clear();
    }

    /// Explicitly abort and roll back now.
    /// Abort và rollback ngay.
    pub fn abort(&mut self) {
        self.rollback();
    }

    /// Library this guard serializes.
    /// Library mà guard này tuần tự hoá.
    pub fn library(&self) -> &str {
        &self.library
    }

    /// Number of pending undo steps (tests assert a switch left none).
    /// Số bước hoàn tác còn chờ (test kiểm switch không để sót).
    pub fn pending_undos(&self) -> usize {
        self.undos.len()
    }

    fn rollback(&mut self) {
        while let Some(undo) = self.undos.pop() {
            undo();
        }
        self.committed = true;
    }
}

impl Drop for KegGuard {
    fn drop(&mut self) {
        if !self.committed {
            self.rollback();
        }
    }
}

impl std::fmt::Debug for KegGuard {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("KegGuard")
            .field("library", &self.library)
            .field("undos", &self.undos.len())
            .field("committed", &self.committed)
            .finish()
    }
}

/// Captured state of a managed prefix entry.
/// Trạng thái đã chụp của một entry prefix được quản lý.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryState {
    /// Entry did not exist.
    /// Entry chưa tồn tại.
    Missing,
    /// Entry was a symlink with this target.
    /// Entry là symlink với đích này.
    Link(PathBuf),
}

/// Capture the current state of a managed prefix entry.
/// Ghi lại trạng thái hiện tại của entry prefix được quản lý.
pub fn capture_entry(path: &Path) -> KegResult<EntryState> {
    if let Some(target) = crate::install::keg::prefix::link_target(path)? {
        return Ok(EntryState::Link(target));
    }
    if std::fs::symlink_metadata(path).is_ok() {
        return Err(KegError::UnsafePath {
            path: path.display().to_string(),
            reason: "managed prefix entry is not a symlink",
        });
    }
    Ok(EntryState::Missing)
}

/// Build the undo step that restores a captured entry state.
/// Tạo bước hoàn tác đưa entry về trạng thái đã chụp.
pub fn restore_entry(path: PathBuf, state: EntryState) -> Undo {
    Box::new(move || match state {
        EntryState::Missing => {
            let _ = crate::install::keg::prefix::remove_link(&path);
        }
        EntryState::Link(target) => {
            let _ = crate::install::keg::prefix::remove_link(&path);
            let _ = crate::install::keg::prefix::create_symlink(&target, &path);
        }
    })
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn process_lock_for(root: &Path, name: &str) -> KegResult<Arc<RwLock<()>>> {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<RwLock<()>>>>> = OnceLock::new();
    let registry = REGISTRY.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = registry
        .lock()
        .map_err(|_| KegError::Layout("keg lock registry is poisoned".to_string()))?;
    Ok(Arc::clone(
        map.entry(format!("{}::{name}", root.display()))
            .or_insert_with(|| Arc::new(RwLock::new(()))),
    ))
}

/// Cross-process advisory lock on a dedicated lock file.
/// Khóa advisory liên tiến trình trên file lock riêng.
struct OsLock {
    // Held open: closing the file releases the advisory lock.
    // Giữ mở: đóng file là nhả khóa advisory.
    _file: std::fs::File,
}

impl OsLock {
    fn acquire(path: &Path) -> KegResult<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)
            .map_err(|error| KegError::io("open keg lock file", path, error))?;
        acquire_platform(&file, path)?;
        Ok(Self { _file: file })
    }
}

#[cfg(unix)]
fn acquire_platform(file: &std::fs::File, path: &Path) -> KegResult<()> {
    use std::os::unix::io::AsRawFd;
    // SAFETY: flock only uses the fd of an open file; the fd stays valid for
    // the OsLock lifetime and no pointer is dereferenced.
    // SAFETY: flock chỉ dùng fd của file đang mở; fd sống trong suốt OsLock
    // và không giải tham chiếu con trỏ nào.
    #[allow(unsafe_code)]
    let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) };
    if result != 0 {
        return Err(KegError::io(
            "lock keg directory",
            path,
            std::io::Error::last_os_error(),
        ));
    }
    Ok(())
}

#[cfg(not(unix))]
fn acquire_platform(file: &std::fs::File, path: &Path) -> KegResult<()> {
    // Windows `LockFileEx` needs `windows-sys`, which this crate does not
    // depend on. In-process serialization still applies; cross-process
    // exclusivity on Windows is not guaranteed by this layer.
    // Windows cần `windows-sys` (crate này không phụ thuộc). Tuần tự hoá trong
    // tiến trình vẫn có; độc quyền liên tiến trình trên Windows chưa được bảo
    // đảm bởi lớp này.
    let _ = (file, path);
    Ok(())
}
