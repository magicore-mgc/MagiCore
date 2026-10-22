//! pub.dev resolver module wiring and stable exports.
//! Chỉ nối module resolver pub.dev và giữ các đường export ổn định.

mod graph_resolution;
mod materialize;
mod package_catalog;
mod registry;
mod registry_client;
mod version_constraints;

#[path = "pub.rs"]
mod pub_implementation;

pub use pub_implementation::PubProtocol;
