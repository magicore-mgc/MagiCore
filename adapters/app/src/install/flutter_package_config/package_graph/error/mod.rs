//! Wire the package-graph error taxonomy and its mapping tests.
//! Nối taxonomy lỗi package graph và test mapping.

mod error;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::error::FlutterPackageGraphError;
