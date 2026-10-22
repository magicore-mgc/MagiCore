//! Top-level project dependency extraction and scope filtering.
//! Trích dependency cấp project và lọc scope.

mod pom_dependencies;
pub use pom_dependencies::collect_pom_dependencies;
