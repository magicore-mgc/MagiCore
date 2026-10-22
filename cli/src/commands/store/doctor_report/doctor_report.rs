//! Defines the machine-readable Web doctor report contract.
//! Định nghĩa hợp đồng báo cáo Web doctor dạng máy đọc.

// Structured result for --json (P0-B): stable machine-readable
// contract, one object, fields stable across releases. Generation
// health fields (Gate 11-B, vòng-11): the doctor now proves token
// invariants, not just "SQLite opens" — orphan claims (no marker),
// stale staging generations (crashed installs), promoted uniqueness
// per project, and referenced-blob existence.
// (Kết quả có cấu trúc cho --json (P0-B): hợp đồng máy đọc ổn định,
// một object, field ổn định qua các release. Field sức khỏe
// generation (Gate 11-B): doctor giờ chứng minh invariant token,
// không chỉ "SQLite mở được" — claim mồ côi (thiếu marker), staging
// generation chết (install đứt), tính duy nhất promoted mỗi project,
// và sự tồn tại của blob được tham chiếu.)
#[derive(serde::Serialize)]
pub(in crate::commands::store) struct DoctorReport {
    pub(in crate::commands::store) core: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::commands::store) framework: Option<String>,
    pub(in crate::commands::store) scope: &'static str,
    pub(in crate::commands::store) integrity_checked: bool,
    pub(in crate::commands::store) store_root: String,
    pub(in crate::commands::store) db_ok: bool,
    pub(in crate::commands::store) cas_root_ok: bool,
    pub(in crate::commands::store) tmp_dir_ok: bool,
    pub(in crate::commands::store) repaired: bool,
    pub(in crate::commands::store) healthy: bool,
    pub(in crate::commands::store) generation_state_ok: bool,
    pub(in crate::commands::store) orphan_claims: usize,
    pub(in crate::commands::store) stale_staging_generations: usize,
    pub(in crate::commands::store) retained_staging_generations: usize,
    pub(in crate::commands::store) missing_referenced_blobs: usize,
    pub(in crate::commands::store) corrupt_referenced_blobs: usize,
}
