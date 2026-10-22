//! RAII guard that aborts an unpromoted generation on drop.
//! Guard RAII hủy generation chưa promote khi bị drop.
use super::super::super::types::Database;

/// RAII generation guard (Gate 11-A, item 6): begin → claims → promote.
/// On ANY early return or panic, Drop aborts the STILL-STAGING token —
/// a crashed install can never leak a staging generation whose claims
/// over-protect blobs forever. `disarm()` after a successful promote
/// cancels the auto-abort; the guard is then a no-op.
///
/// The guard deliberately holds NO borrow of the Database (its caller
/// flows through async code where a borrowed guard crossing an await
/// makes the future !Send): Drop opens its OWN short-lived connection to
/// the same store DB and aborts there. Semantics match crash recovery —
/// if the process dies before Drop, the staging generation leaks to the
/// doctor's stale-staging GC, which is the same cleanup path.
/// (Guard generation RAII: begin → claim → promote. Mọi return sớm hay
/// panic, Drop hủy token VẪN-STAGING — install đứt không bao giờ rò
/// staging generation mà claim của nó giữ blob vô hạn. `disarm()` sau
/// promote thành công hủy auto-abort; guard khi đó là no-op.
/// Guard CỐ Ý không giữ borrow của Database (caller chảy qua async,
/// guard giữ borrow vượt await làm future !Send): Drop mở connection
/// RIÊNG tạm thời đến cùng store DB và hủy ở đó. Ngữ nghĩa khớp phục
/// hồi crash — process chết trước Drop thì staging generation rò tới GC
/// stale-staging của doctor, cùng đường dọn.)
pub struct CasGenerationGuard {
    pub(in crate::database::database::generation_guard) db_path: std::path::PathBuf,
    pub(in crate::database::database::generation_guard) project_root: String,
    pub(in crate::database::database::generation_guard) generation: i64,
    pub(in crate::database::database::generation_guard) armed: bool,
}

impl std::fmt::Debug for CasGenerationGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CasGenerationGuard")
            .field("project_root", &self.project_root)
            .field("generation", &self.generation)
            .field("armed", &self.armed)
            .finish()
    }
}

impl Drop for CasGenerationGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        // Fail-closed cleanup: a failed abort leaves the staging token to
        // the doctor's stale-staging GC — warned, never panicked (the guard
        // runs on drop paths). English-only console (RULE §7).
        // (Dọn fail-closed: abort thất bại để lại token staging cho GC
        // stale-staging của doctor — cảnh báo, không panic (guard chạy trên
        // đường drop). Console tiếng Anh (RULE §7).)
        match Database::open(&self.db_path)
            .map(|db| db.abort_cas_generation(&self.project_root, self.generation))
        {
            Ok(Ok(())) => {}
            Ok(Err(e)) => eprintln!(
                "warning: failed to auto-abort cas generation {} for '{}': {e} \
                 (stale staging claims remain until doctor)",
                self.generation, self.project_root
            ),
            Err(e) => eprintln!(
                "warning: failed to open store db '{}' for auto-abort of cas \
                 generation {}: {e} (stale staging claims remain until doctor)",
                self.db_path.display(),
                self.generation
            ),
        }
    }
}

impl CasGenerationGuard {
    /// Disarm after a successful promote — the token is promoted, its
    /// claims are live; drop must NOT abort it.
    /// (Disarm sau promote thành công — token đã promoted, claim của
    /// nó live; drop KHÔNG được hủy nó.)
    pub fn disarm(mut self) {
        self.armed = false;
    }

    pub fn generation(&self) -> i64 {
        self.generation
    }
}
