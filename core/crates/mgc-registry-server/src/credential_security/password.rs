//! Argon2id policy v1; stored parameters cannot choose unbounded KDF work.
//! Policy Argon2id v1; tham số lưu trữ không được chọn KDF vô hạn tài nguyên.

use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use super::CredentialError;

fn policy_parameters() -> Result<Params, CredentialError> {
    Params::new(19_456, 2, 1, Some(32)).map_err(|_| CredentialError::Parameters)
}

fn hasher() -> Result<Argon2<'static>, CredentialError> {
    Ok(Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        policy_parameters()?,
    ))
}

/// Always hash plaintext, including plaintext resembling a PHC value.
/// Luôn hash plaintext, kể cả plaintext có hình thức giống PHC.
pub(crate) fn hash_password(password: &str) -> Result<String, CredentialError> {
    hasher()?
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|_| CredentialError::Hashing)
}

fn parse_hash(encoded: &str) -> Option<PasswordHash> {
    let hash = PasswordHash::new(encoded).ok()?;
    let parameters = Params::try_from(&hash).ok()?;
    (hash.algorithm == Algorithm::Argon2id.ident()
        && hash.version == Some(19)
        && parameters == policy_parameters().ok()?
        && hash.salt.as_ref().is_some_and(|salt| salt.len() >= 16))
    .then_some(hash)
}

pub(crate) fn valid_password_hash(encoded: &str) -> bool {
    parse_hash(encoded).is_some()
}

pub(crate) fn verify_password_hash(password: &str, encoded: &str) -> bool {
    let Some(hash) = parse_hash(encoded) else {
        return false;
    };
    hasher().is_ok_and(|argon2| argon2.verify_password(password.as_bytes(), &hash).is_ok())
}
