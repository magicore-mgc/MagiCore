use super::super::{errors::CasGenerationError, types::Database};
use super::ProjectInstallLock;

impl Database {
    /// Project-level install lock (Gate 11-A, item 7 — vòng-11 verdict):
    /// one install per project at a time. SQLite generation tokens keep
    /// CAS claims crash-safe, but two installs mutating the SAME
    /// node_modules/lockfile in parallel is a correctness hazard the
    /// refcount protocol cannot fix (whoever renames last wins the
    /// tree). Production package managers serialize project mutation the
    /// same way. The lock is an exclusive OS file lock (fs2) held for
    /// the guard's lifetime — cross-process safe; a crashed process
    /// releases it automatically (OS closes the fd).
    /// (Khóa install mức project: một install mỗi project tại một thời
    /// điểm. Token generation SQLite giữ claim CAS an toàn khi crash,
    /// nhưng 2 install song song mutate cùng node_modules/lockfile là mối
    /// nguy correctness mà giao thức refcount không thể sửa (ai rename
    /// sau cùng thắng cả cây). Package manager production tuần tự hóa
    /// mutation project theo cách này. Khóa là file lock độc quyền OS
    /// (fs2) giữ suốt đời guard — an toàn cross-process; tiến trình đứt
    /// tự nhả (OS đóng fd).)
    pub fn project_install_lock(
        &self,
        project_root: &str,
    ) -> std::result::Result<ProjectInstallLock, CasGenerationError> {
        ProjectInstallLock::acquire(project_root)
    }
}
