//! Lib manifest context wiring — wiring context manifest Lib.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod lib_project_context;
pub use lib_project_context::lib_project_context;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
