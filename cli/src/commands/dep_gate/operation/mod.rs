//! Operation vocabulary wiring — wiring vocabulary operation.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod operation;
pub use operation::DepOp;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
