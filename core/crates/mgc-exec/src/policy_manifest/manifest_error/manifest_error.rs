/// Verification failure — lỗi xác minh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    /// Unknown schema version — phiên bản schema không nhận diện.
    UnknownSchema(String),
    /// Signing key is not in the trusted set — khóa ký không được tin.
    UntrustedKey,
    /// Signing key was revoked — khóa ký đã bị thu hồi.
    RevokedKey,
    /// A revoked key id is not a canonical lowercase BLAKE3 hex id.
    /// Key ID bị thu hồi không phải BLAKE3 hex chữ thường chuẩn.
    InvalidRevocationKey,
    /// Bad signature — chữ ký sai.
    BadSignature,
    /// Past expiry — đã hết hạn.
    Expired,
    /// Manifest was issued in the future — manifest chưa tới thời điểm hiệu lực.
    NotYetValid,
    /// Expiry must be strictly later than issue time — expires_at phải sau issued_at.
    InvalidTimeWindow,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
