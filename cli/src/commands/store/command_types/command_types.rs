//! Owns store CLI subcommands and the stable prune report shape.
//! Sở hữu subcommand store và cấu trúc báo cáo prune ổn định.

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum StoreCmd {
    /// Delete packages not referenced by any project (refcount == 0)
    Prune {
        #[arg(long, help = "report only, do not delete")]
        dry_run: bool,
        #[arg(long, help = "output JSON")]
        json: bool,
    },
    /// Review and reclaim package store bytes no project references anymore.
    /// Lists unused packages with their size first; deleting needs --apply
    /// and then --yes. Reports only by default.
    /// (Rà và thu hồi dung lượng store không còn project nào dùng. Liệt kê
    /// kèm dung lượng trước; xoá cần --apply rồi --yes. Mặc định chỉ báo cáo.)
    Reclaim {
        #[arg(
            long,
            help = "plan deletion after listing (still needs --yes to delete)"
        )]
        apply: bool,
        #[arg(long, help = "confirm deletion together with --apply")]
        yes: bool,
        #[arg(long, help = "output JSON")]
        json: bool,
    },
    /// Show package store summary
    Status,
    /// Report cache/store measurements (blobs, bytes, memo hit/miss, dedupe).
    /// Báo số đo cache/store (blob, byte, memo hit/miss, dedupe).
    ///
    /// Every number is read from the live store; nothing is estimated, so the
    /// output is the only allowed source for a cache claim.
    /// (Mọi con số đọc từ store thật; không ước lượng, nên output là nguồn duy
    /// nhất được dùng cho claim về cache.)
    Stats {
        /// Emit a stable machine-readable object.
        /// Xuất object máy đọc ổn định.
        #[arg(long, help = "output JSON")]
        json: bool,
        /// Store root to inspect (default: this project's store).
        /// Root store cần đo (mặc định: store của project này).
        #[arg(long, value_name = "PATH")]
        root: Option<String>,
    },
    /// Back up the store to a target directory
    Backup {
        /// Target directory (default: ~/.magicore/store_backup_<timestamp>)
        #[arg(long)]
        path: Option<String>,
    },
    /// Restore the store from a backup directory
    Restore {
        /// Backup directory path
        #[arg(long)]
        path: String,
    },
    /// Check store health (DB readability, CAS root safety) and report
    /// remediation steps — the target of every prune refusal (P1-6 +
    /// P0-C, Tech Lead vòng-7). `--repair` re-locks a quarantined store
    /// and clears stale temp files. EXIT CONTRACT (P0-B, adversarial
    /// review vòng-9): 0 = healthy (or repaired AND re-checked healthy),
    /// 1 = corruption/invariant violation — scripts and CI must be able
    /// to trust the code, `doctor && prune` must not run on a broken
    /// store. `--json` emits a machine-readable status object. Web doctor
    /// checks DB/CAS invariants; other native routes check managed-root
    /// path safety only. Supply `--framework` for non-Web routes.
    /// (Kiểm tra sức khỏe store (đọc được DB, an toàn root CAS) và báo
    /// các bước sửa — đích của mọi lời từ chối prune (P1-6 + P0-C).
    /// `--repair` khóa lại store bị cách ly và dọn temp sót. HỢP ĐỒNG
    /// EXIT (P0-B): 0 = khỏe (hoặc đã repair VÀ re-check khỏe), 1 =
    /// hỏng/vi phạm invariant — script và CI phải tin được exit code,
    /// `doctor && prune` không được chạy trên store hỏng. `--json` xuất
    /// đối tượng trạng thái máy đọc được. Web doctor kiểm tra invariant
    /// DB/CAS; route native khác chỉ kiểm tra an toàn root. Dùng
    /// `--framework` cho route ngoài Web.)
    Doctor {
        #[arg(long, help = "attempt automatic repair, not just a check")]
        repair: bool,
        #[arg(
            long,
            default_value = "web",
            help = "project core whose store route should be checked"
        )]
        core: String,
        #[arg(
            long,
            help = "framework/language store route (required for non-Web routes)"
        )]
        framework: Option<String>,
        #[arg(long, help = "emit machine-readable JSON status")]
        json: bool,
    },
}

#[derive(Debug, serde::Serialize)]
pub struct ReclaimReport {
    /// Unreferenced packages with their on-disk size.
    /// Package không còn tham chiếu kèm dung lượng trên đĩa.
    pub candidates: Vec<ReclaimCandidate>,
    pub total_bytes: u64,
    pub deleted: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct ReclaimCandidate {
    pub name: String,
    pub bytes: u64,
}

#[derive(Debug, serde::Serialize)]
pub struct PruneReport {
    pub unreferenced: Vec<String>,
    pub removed: Vec<String>,
    pub removed_bytes: u64,
    pub dry_run: bool,
}
