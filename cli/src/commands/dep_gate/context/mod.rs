//! Gate context wiring — wiring context gate.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod context;
pub use context::DepContext;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
