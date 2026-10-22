/// One staging generation's crash-recovery lease (Gate 11-B, vòng-11):
/// WHO holds it (pid), WHEN it began, and how many claims it filed. The
/// doctor decides garbage vs live-install from ALL THREE — never from
/// age alone (an install can legitimately run for hours; a dead pid is
/// garbage the instant its claims are zero... but a dead pid WITH
/// claims may still over-protect blobs until its own abort — the doctor
/// reports those, repair is conservative).
/// (Lease phục hồi crash của một staging generation: AI giữ nó (pid),
/// KHI bắt đầu, và đã ghi bao nhiêu claim. Doctor quyết định rác-hay-
/// đang-chạy từ CẢ BA — không bao giờ chỉ theo tuổi (install hợp pháp
/// có thể chạy hàng giờ; pid chết là rác ngay khi claim bằng 0... nhưng
/// pid chết CÓ claim vẫn có thể đang bảo vệ thừa blob tới khi abort của
/// chính nó — doctor báo những cái đó, repair thiên về bảo toàn).)
#[derive(Debug, Clone)]
pub struct StagingLease {
    pub project_root: String,
    pub generation: i64,
    pub lease_pid: Option<i64>,
    pub lease_started_at: Option<i64>,
    pub claim_count: i64,
}
