//! HTML and HMR route owner wiring.
//! Wiring owner cho route HTML và HMR.

#[path = "hmr_client/mod.rs"]
mod hmr_client;
#[path = "index_document/mod.rs"]
mod index_document;

pub(in crate::bundler::dev_server::implementation) use hmr_client::serve_hmr_client;
pub(in crate::bundler::dev_server::implementation) use index_document::serve_index;
