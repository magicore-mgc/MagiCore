//! Finding Class owner exports — xuất kiểu phân loại phát hiện.

mod finding_class;
pub use finding_class::FindingClass;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
