//! CLI bundler compatibility exports and CLI-owned development services.
//! Export tương thích bundler cho CLI cùng các service phát triển thuộc CLI.

pub mod dev_server;

pub use mgc_web_adapter::bundler::{Bundler, BundlerConfig, process_assets};
