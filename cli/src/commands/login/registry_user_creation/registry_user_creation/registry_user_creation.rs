//! Build and send the bounded registry user-creation request.
//! Dựng và gửi request tạo user registry có giới hạn.

use anyhow::Result;

use super::super::constants::{MAX_REGISTRY_AUTH_RESPONSE_BYTES, REGISTRY_AUTH_TIMEOUT_SECS};
use super::super::error::RegistryUserCreationError;
use super::super::registry_user_url::registry_user_url;
use super::super::token_response::parse_token_response;

pub(crate) async fn create_registry_user(
    registry: &str,
    username: &str,
    password: &str,
    extra_fields: &serde_json::Value,
) -> Result<String> {
    create_registry_user_inner(registry, username, password, extra_fields)
        .await
        .map_err(super::super::registration_diagnostic)
}

pub(in crate::commands::login) async fn create_registry_user_inner(
    registry: &str,
    username: &str,
    password: &str,
    extra_fields: &serde_json::Value,
) -> Result<String, RegistryUserCreationError> {
    let url = registry_user_url(registry, username)?;
    let body = registration_body(username, password, extra_fields);
    let client = crate::commands::registry::registry_url::client_for_url(
        &url,
        std::time::Duration::from_secs(REGISTRY_AUTH_TIMEOUT_SECS),
    )
    .map_err(|_| RegistryUserCreationError::RequestFailed)?;
    let response = client
        .put(url)
        .json(&body)
        .send()
        .await
        .map_err(|_| RegistryUserCreationError::RequestFailed)?;
    let status = response.status();
    if !status.is_success() {
        return Err(RegistryUserCreationError::StatusFailed(status.as_u16()));
    }
    if response
        .content_length()
        .is_some_and(|size| size > MAX_REGISTRY_AUTH_RESPONSE_BYTES as u64)
    {
        return Err(RegistryUserCreationError::ResponseTooLarge);
    }
    let mut response = response;
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| RegistryUserCreationError::RequestFailed)?
    {
        if bytes.len().saturating_add(chunk.len()) > MAX_REGISTRY_AUTH_RESPONSE_BYTES {
            return Err(RegistryUserCreationError::ResponseTooLarge);
        }
        bytes.extend_from_slice(&chunk);
    }
    parse_token_response(&bytes)
}

fn registration_body(
    username: &str,
    password: &str,
    extra_fields: &serde_json::Value,
) -> serde_json::Value {
    let mut body = serde_json::json!({
        "name": username,
        "password": password,
    });
    if let (Some(body), Some(extra)) = (body.as_object_mut(), extra_fields.as_object()) {
        body.extend(
            extra
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
    }
    body
}

#[cfg(test)]
#[path = "test/registry_user_creation.rs"]
mod tests;
