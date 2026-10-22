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
    /// Show package store summary
    Status,
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
pub struct PruneReport {
    pub unreferenced: Vec<String>,
    pub removed: Vec<String>,
    pub removed_bytes: u64,
    pub dry_run: bool,
}
