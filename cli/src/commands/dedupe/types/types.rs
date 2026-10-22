//! Dedupe request and report types — keeps command input and JSON fields stable.
//! Kiểu request và report dedupe — giữ ổn định input command và field JSON.
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct DedupeArgs {
    #[arg(long, help = "report only, do not apply changes")]
    pub dry_run: bool,
    #[arg(long, help = "prefer latest version over existing instances")]
    pub prefer_latest: bool,
    #[arg(long, help = "output JSON")]
    pub json: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct DedupeReport {
    pub before_instances: usize,
    pub after_instances: usize,
    pub merged: usize,
    /// Legacy JSON key; this is observed logical payload in removed paths,
    /// not filesystem blocks or a guarantee of physical disk reclamation.
    /// (Tên JSON cũ; đây là payload logic quan sát được trong path đã xóa,
    /// không phải số block hay dung lượng vật lý chắc chắn được thu hồi.)
    pub disk_saved_bytes: u64,
    pub entries: Vec<DedupeEntry>,
    /// GC paths that failed deletion (bytes NOT counted above).
    /// Empty means the GC pass was complete.
    /// (Đường GC lỗi — byte chưa xóa không được cộng.)
    pub gc_failed_paths: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct DedupeEntry {
    pub package: String,
    pub version: String,
    pub instances_before: usize,
    pub instances_after: usize,
    pub action: String,
}
