//! Configured registry credential owner / owner credentials registry trong config.

mod registry_credentials;
#[cfg(test)]
mod test;

pub(in crate::auth::resolve_auth) use registry_credentials::resolve_registry_credentials;
