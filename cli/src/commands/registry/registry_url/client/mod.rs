//! Registry client owner.
//! Owner client registry.

mod client;

#[cfg(test)]
pub(crate) use client::should_bypass_system_proxy;
pub(crate) use client::{admin_client, client_for_url};
