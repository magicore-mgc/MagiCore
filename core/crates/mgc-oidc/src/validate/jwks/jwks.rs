//! JWKS key and set types — kiểu khóa và tập khóa JWKS.
//! Kiểu serializable dùng chung giữa fetch, cache và JWT verifier.

use serde::{Deserialize, Serialize};

/// One JSON Web Key (RSA only — the only algorithm this verifier accepts).
/// Một khóa JSON Web Key; verifier này chỉ chấp nhận RSA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwk {
    /// Key type — must be `"RSA"`.
    /// Loại khóa — bắt buộc là `"RSA"`.
    pub kty: String,
    /// Key id — must match the JWT header `kid`.
    /// ID khóa — phải khớp `kid` trong header JWT.
    #[serde(default)]
    pub kid: Option<String>,
    /// Base64url modulus.
    /// Modulus mã hóa Base64url.
    pub n: String,
    /// Base64url exponent.
    /// Exponent mã hóa Base64url.
    pub e: String,
}

/// A JSON Web Key Set document.
/// Tài liệu tập hợp các JSON Web Key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwks {
    /// Signing keys.
    /// Các khóa dùng để ký.
    pub keys: Vec<Jwk>,
}
