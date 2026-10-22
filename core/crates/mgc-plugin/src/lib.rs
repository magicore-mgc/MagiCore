#![forbid(unsafe_code)]

//! Plugin architecture — tách pipeline thành 5 trait (T3).
//!
//! Back-compat: `PackageAdapter` monolith (mgc-types) giữ nguyên; adapter cũ
//! map lên qua `AsPlugin::as_plugin()` — 0 hành vi đổi. Plugin mới đi 5 trait
//! trực tiếp.

pub mod backend;
pub mod fetcher;
pub mod linker;
pub mod resolver;
pub mod template;

pub use backend::CacheBackend;
pub use fetcher::Fetcher;
pub use linker::Linker;
pub use resolver::Resolver;
pub use template::TemplateGenerator;

#[path = "plugin/mod.rs"]
mod plugin;
pub use plugin::Plugin;

#[path = "registry/mod.rs"]
mod registry;
pub use registry::{PluginRegistry, global, register};
