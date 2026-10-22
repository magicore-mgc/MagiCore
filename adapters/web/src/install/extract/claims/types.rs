//! Typed install-generation claim context shared by extraction owners.
//! Kiểu context claim theo generation dùng chung giữa owner extraction.
use mgc_store::Database;

pub struct CasClaimContext {
    /// Fail-closed DB handle (Gate 11-A, P0-5 of vòng-11 audit): the OLD
    /// field was `Result<Database, String>` and `as_ref()` folded a DB
    /// open failure into `None` — extraction continued WITHOUT claims,
    /// then the end-of-install promote retired previous claims: warm CAS
    /// silently became unreferenced. Now the constructor RETURNS
    /// `Result` and every caller propagates the error — an install that
    /// cannot open the claim DB FAILS, it never claims nothing.
    /// (Handle DB fail-closed (P0-5): field cũ là
    /// `Result<Database, String>` và `as_ref()` gộp lỗi mở DB thành
    /// `None` — extraction tiếp tục KHÔNG claim, rồi promote cuối install
    /// nghỉ hưu claim cũ: warm CAS âm thầm mất tham chiếu. Giờ constructor
    /// TRẢ `Result` và mọi caller propagate lỗi — install không mở được DB
    /// claim thì FAIL, không bao giờ "claim không gì".)
    pub db: Database,
    pub project_key: String,
    /// Install generation token (P0-A): every claim this context files lands
    /// in THIS generation — begin/promote/abort all key off it, never off a
    /// global MAX(generation).
    /// (Token generation của install (P0-A): mọi claim qua context này rơi
    /// vào generation NÀY — begin/promote/abort đều khóa theo nó, không bao
    /// giờ theo MAX(generation) toàn cục.)
    pub generation: i64,
}

impl CasClaimContext {
    pub fn as_ref(&self) -> (&Database, &str, i64) {
        (&self.db, &self.project_key, self.generation)
    }
}
