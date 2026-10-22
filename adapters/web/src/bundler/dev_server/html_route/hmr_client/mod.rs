//! HMR client route owner wiring.
//! Wiring owner route HMR client.

#[path = "hmr_client.rs"]
mod implementation;

pub(in crate::bundler::dev_server::implementation) use implementation::serve_hmr_client;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
