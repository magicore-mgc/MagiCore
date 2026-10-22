//! Build a registry user endpoint through the shared URL policy.
//! Tạo endpoint user theo URL policy dùng chung.

use url::Url;

use super::super::error::RegistryUserCreationError;

pub(in crate::commands::login::registry_user_creation) fn registry_user_url(
    registry: &str,
    username: &str,
) -> Result<Url, RegistryUserCreationError> {
    crate::commands::registry::registry_url::user_url(registry, username).map_err(|error| {
        match error {
            crate::commands::registry::registry_url::RegistryUrlError::RequiresHttps => {
                RegistryUserCreationError::RequiresHttps
            }
            _ => RegistryUserCreationError::InvalidUrl,
        }
    })
}

#[cfg(test)]
#[path = "test/registry_user_url.rs"]
mod tests;
