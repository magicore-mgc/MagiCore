//! Dependency cache owner wiring and public exports.
//! Wiring owner cache dependency và export public.

mod build;
mod dependency_cache;
mod error;
mod name_validation;
mod package_entry;
mod types;

pub use dependency_cache::DepsCache;
pub use name_validation::dep_name_is_valid;
pub use types::CachedDep;
