//! Dependency declaration option validation for Cargo projection.
//! Kiểm tra option khai báo dependency khi projection sang Cargo.

mod dependency_specs;
pub(super) use dependency_specs::validate_dependency_specs;
