//! Installed package identity and marker verification.
//! Xác minh định danh package đã materialize và marker của nó.

mod materialized_package;

pub use materialized_package::{materialized_package_matches, write_materialized_package_marker};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
