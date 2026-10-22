//! Web bundler error wiring.
//! Wiring lỗi Web bundler.

mod error;

pub(super) use error::{esbuild_failed, link_node_modules_failed};
