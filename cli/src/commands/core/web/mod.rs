//! Web core command facade — export web lifecycle commands.
//! Facade lệnh core web — export lệnh lifecycle web.

#[path = "dependency_why/mod.rs"]
mod dependency_why;
#[path = "web.rs"]
mod implementation;
#[path = "links/mod.rs"]
mod links;
pub use crate::commands::web_scaffold_flags as scaffold_flags;

pub use dependency_why::why;
pub use implementation::*;
pub(crate) use implementation::{enrich_web_project_manifest, parse_framework_request};
pub use links::{link, unlink};
