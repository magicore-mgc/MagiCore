//! Wire complete-graph edge validation and local tests.
//! Nối kiểm tra edge toàn graph và test tại chỗ.

mod edge_validation;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::edge_validation::validate_package_edges;
