//! Topological-order typed-error facade.
//! Facade lỗi có kiểu của sắp thứ tự topo.

mod error;

pub use error::TopoError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
