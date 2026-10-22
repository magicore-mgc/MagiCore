//! Stable Bun/Deno scanner facade — giữ facade scanner Bun/Deno ổn định.

/// One npm package pin extracted from a Bun/Deno lockfile.
/// Một ghim package npm trích từ lockfile Bun/Deno.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmPin {
    pub name: String,
    pub version: String,
}

#[path = "bun_deno/bun_lock/mod.rs"]
mod bun_lock;
#[path = "bun_deno/deno_lock/mod.rs"]
mod deno_lock;
#[path = "bun_deno/package_spec/mod.rs"]
mod package_spec;
#[path = "bun_deno/project_lockfiles/mod.rs"]
mod project_lockfiles;
#[path = "bun_deno/types/mod.rs"]
mod types;

pub use bun_lock::read_bun_lock;
pub use deno_lock::read_deno_lock;
use package_spec::split_name_version;
pub use project_lockfiles::read_js_lockfiles;
pub use types::{BunDenoRead, JrSkipped};
