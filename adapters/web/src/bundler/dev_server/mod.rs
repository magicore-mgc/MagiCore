//! Public Web dev-server boundary and owner exports.
//! Boundary public dev-server Web và export owner.

#[path = "dev_server.rs"]
mod implementation;

pub use implementation::{DevServerConfig, MgDevServer};
