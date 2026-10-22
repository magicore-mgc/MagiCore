//! Public result values for lockfile signature and trust verification.
//! Giá trị kết quả public cho verify chữ ký và trust lockfile.

/// Verification result — Kết quả verify.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationStatus {
    /// Lockfile is valid and signed — Lockfile hợp lệ và đã ký.
    Valid,
    /// Lockfile is unsigned (warning) — Lockfile chưa ký (cảnh báo).
    Unsigned,
    /// Lockfile has been tampered with — Lockfile đã bị tamper.
    Tampered(String),
    /// Signature is invalid — Chữ ký không hợp lệ.
    InvalidSignature(String),
    /// Signature is valid, but the signer is outside project trust roots.
    /// Chữ ký hợp lệ nhưng signer nằm ngoài trust roots của project.
    UntrustedKey(String),
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
