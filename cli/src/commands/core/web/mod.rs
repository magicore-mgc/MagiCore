//! Web core command facade — export web lifecycle commands.
//! Facade lệnh core web — export lệnh lifecycle web.

#[path = "dependency_why/mod.rs"]
mod dependency_why;
#[path = "web.rs"]
mod implementation;
#[path = "install_layout/mod.rs"]
mod install_layout;
#[path = "links/mod.rs"]
mod links;
pub mod scaffold_flags;

pub use dependency_why::why;
pub use implementation::*;
pub(crate) use implementation::{enrich_web_project_manifest, parse_framework_request};
pub use install_layout::should_use_legacy_flat_layout;
pub use links::{link, unlink};
