//! Structural verification result, separate from trust-policy decisions.
//! Kết quả xác minh cấu trúc, tách khỏi quyết định trust policy.

/// Structural verification report: digest math + signature math, NO
/// policy. Policy enforcement is a separate explicit step below.
/// Báo cáo verify cấu trúc: toán digest + chữ ký, KHÔNG policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V4VerifyReport {
    /// Payload digest recomputed and equal to `metadata.lockfile_hash`.
    pub digest_ok: bool,
    /// A syntactically valid signature block is present.
    pub signed: bool,
    /// The signature verifies against the embedded public key.
    pub signature_ok: bool,
    /// Key id from the signature block, if any.
    pub key_id: Option<String>,
}
