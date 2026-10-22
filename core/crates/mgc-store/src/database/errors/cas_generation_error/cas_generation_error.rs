/// Typed generation-token errors (Gate 11-A, vòng-11 audit): every
/// mutation of the token protocol must VERIFY the token BEFORE any row
/// is written — an unknown/forged/stale token is a typed error with
/// ZERO mutation, never a silent partial retire. This is the
/// "untrusted caller" contract: a corrupt or stale token can never
/// retire another generation's claims.
/// (Lỗi token generation có type (Gate 11-A): mọi mutation của giao
/// thức token phải VERIFY token TRƯỚC khi ghi row — token không
/// biết/giả/cũ là lỗi có type với KHÔNG mutation nào, không bao giờ
/// nghỉ hưu một phần âm thầm. Đây là hợp đồng "caller không tin
/// cậy": token hỏng hoặc cũ không bao giờ được nghỉ hưu claim của
/// generation khác.)
#[derive(Debug, thiserror::Error)]
pub enum CasGenerationError {
    #[error("cas generation token {generation} is not registered for project '{project_root}'")]
    UnknownToken {
        project_root: String,
        generation: i64,
    },
    #[error("cas generation token {generation} for project '{project_root}' is already promoted")]
    AlreadyPromoted {
        project_root: String,
        generation: i64,
    },
    #[error("database I/O error in cas generation protocol: {0}")]
    Io(String),
}
