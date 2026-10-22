//! Cargo owner exports — export owner cargo.

mod cargo;
pub use cargo::{check_cargo_freshness, check_lib_cargo_freshness, declared_cargo_deps};
