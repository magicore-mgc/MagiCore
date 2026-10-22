//! Import descriptors and reports — descriptor và báo cáo import.

use std::path::PathBuf;

/// Legacy lockfile descriptor — mô tả 1 lockfile PM tìm thấy trong project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyLockfile {
    pub file_name: &'static str,
    pub path: PathBuf,
}

/// Supported legacy lockfile names — tên file được nhận dạng.
pub const NPM_LOCKFILE: &str = "package-lock.json";
pub const PNPM_LOCKFILE: &str = "pnpm-lock.yaml";
pub const YARN_LOCKFILE: &str = "yarn.lock";
pub const BUN_LOCKFILE: &str = "bun.lock";
pub const DENO_LOCKFILE: &str = "deno.lock";

pub const ALL: [&str; 5] = [
    NPM_LOCKFILE,
    PNPM_LOCKFILE,
    YARN_LOCKFILE,
    BUN_LOCKFILE,
    DENO_LOCKFILE,
];

/// Kết quả import — báo cáo nguồn + số package đã chuyển đổi + cảnh báo phiên bản.
/// Skipped records (P0-5 2026-09-11): mọi entry bị bỏ qua phải được ghi
/// đích danh kèm lý do — KHÔNG có skip âm thầm. Đổi tên 2026-09-12 (P0
/// finding #8): migration là BEST-EFFORT kèm loss report tường minh,
/// KHÔNG phải lossless — bun/deno lưu RANGES/prefixes mà mgc.lock chỉ
/// giữ pin; mọi thông tin không giữ được phải xuất hiện ở đây.
// (Import outcome — source file, converted package count, version advisories.
// Skipped records: every skipped entry must be named with a reason —
// silent skips are gone. Rename 2026-09-12 (P0 finding #8): migration is
// BEST-EFFORT with an explicit loss report, NOT lossless — bun/deno store
// RANGES/prefixes while mgc.lock keeps pins; anything not preserved must
// appear in this report.)
#[derive(Debug, Clone)]
pub struct ImportReport {
    pub source_file: String,
    pub packages: usize,
    /// Cảnh báo phiên bản format mới hơn mức đã kiểm chứng (parse theo shape).
    // (Warnings for format versions newer than tested — imported by structure.)
    pub warnings: Vec<String>,
    /// Records the importer refused or could not map — recorded
    /// EXPLICITLY so `mgc import` can surface them (P0-5: best-effort
    /// migration — every input record is either imported or listed;
    /// the report IS the loss ledger, so "lossless" is never claimed).
    /// Record importer từ chối hoặc không ánh xạ được — liệt kê
    /// TƯỜNG MINH để `mgc import` hiển thị (P0-5: migration best-effort
    /// — mọi record input hoặc được import hoặc được liệt kê; báo cáo
    /// chính là sổ ghi mất mát, không bao giờ claim "lossless").
    pub skipped: Vec<SkippedRecord>,
}

/// One record the importer refused, with the exact reason.
/// Một record bị importer từ chối, kèm lý do chính xác.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedRecord {
    pub key: String,
    pub reason: String,
}
