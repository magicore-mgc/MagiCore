//! `cache/mod.rs` — Cache management for app adapter facade.
//! Facade quản lý cache cho adapter App.
//!
//! S08 removed the dead `metadata`/`prune` re-exports of lib cache items: they
//! had zero callers (app uses its own AppLanguage-typed cache functions) and
//! cross-core `pub use` re-exports violate R07 one-way direction.
//! S08 đã xóa 2 re-export chết của lib cache: không caller nào dùng (app có hàm
//! cache riêng theo AppLanguage), và re-export `pub use` xuyên core vi phạm R07.

mod clear;
mod path;
mod size;

pub use clear::clear_cache;
pub use path::cache_dir;
pub use size::cache_size;
#[cfg(test)]
pub(crate) use size::dir_size;

#[cfg(test)]
#[path = "test/cache_size.rs"]
mod cache_size_tests;
