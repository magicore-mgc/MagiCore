//! OCI client facade and registry-operation owners.
//! Facade OCI client và các owner thao tác registry.

mod blob;
mod catalog;
mod client;
mod error;
mod manifest_ops;
mod model;

pub use client::OciClient;
