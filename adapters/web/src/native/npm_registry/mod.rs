// npm registry owner modules. / Modules propriétaires du registre npm.
pub(in crate::native::npm_registry) mod npm_registry;
#[cfg(test)]
use std::time::Duration;
pub(in crate::native::npm_registry) mod batch;
pub(in crate::native::npm_registry) mod client;
pub(in crate::native::npm_registry) mod integrity;
pub(in crate::native::npm_registry) mod metadata;
pub(in crate::native::npm_registry) mod registry;
pub(in crate::native::npm_registry) mod retry;
pub(in crate::native::npm_registry) mod security;
pub(in crate::native::npm_registry) mod types;
pub(in crate::native::npm_registry) use client::client::global_http_client;
pub(in crate::native::npm_registry) use client::diagnostic::diagnostic::client_builder_error;
pub(in crate::native::npm_registry) use client::profile::profile::network_profile_log;
pub(in crate::native::npm_registry) use integrity::integrity::base64_encode;
pub(in crate::native::npm_registry) use metadata::metadata::metadata_accept_header;
pub use npm_registry::*;
pub(in crate::native::npm_registry) use retry::retry::with_retry;
pub(in crate::native::npm_registry) use security::credential_scope::path::path::{
    SCOPED_REGISTRY_TIMEOUT_SECONDS, credential_path_matches,
};
pub(in crate::native::npm_registry) use security::origin::origin::registry_origin_matches;
pub(in crate::native::npm_registry) use security::redirect::redirect::registry_redirect_policy;
pub(in crate::native::npm_registry) use security::security::is_auth_error;
#[cfg(test)]
#[path = "../test/npm_registry_tests.rs"]
mod tests;
