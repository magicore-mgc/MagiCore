//! Wire the registry user URL builder.
//! Nối bộ dựng URL user registry.

#[path = "registry_user_url.rs"]
mod registry_user_url;

pub(super) use registry_user_url::registry_user_url;
