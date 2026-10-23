//! Shared registry URL validation and safe HTTP client construction.
//! Kiểm tra URL registry dùng chung và tạo HTTP client an toàn.

mod error;
mod registry_url;

pub(crate) use error::RegistryUrlError;
pub(crate) use registry_url::{
    admin_client, binding_url, client_for_url, is_loopback_host, revoke_token_url, user_url,
    validated_registry_base,
};
