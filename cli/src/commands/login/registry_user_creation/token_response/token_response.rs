//! Decode and validate registry token responses.
//! Giải mã và xác thực token response từ registry.

use super::super::constants::MAX_REGISTRY_AUTH_RESPONSE_BYTES;
use super::super::error::RegistryUserCreationError;

pub(in crate::commands::login) fn parse_token_response(
    bytes: &[u8],
) -> Result<String, RegistryUserCreationError> {
    if bytes.len() > MAX_REGISTRY_AUTH_RESPONSE_BYTES {
        return Err(RegistryUserCreationError::ResponseTooLarge);
    }
    let parsed: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| RegistryUserCreationError::ResponseInvalid)?;
    let token = parsed["token"]
        .as_str()
        .filter(|token| {
            !token.is_empty()
                && token.len() <= MAX_REGISTRY_AUTH_RESPONSE_BYTES
                && token
                    .bytes()
                    .all(|byte| !byte.is_ascii_whitespace() && !byte.is_ascii_control())
        })
        .ok_or(RegistryUserCreationError::TokenMissing)?;
    Ok(token.to_owned())
}

#[cfg(test)]
#[path = "test/token_response.rs"]
mod tests;
