//! Runtime ownership gate wiring — wiring gate ownership runtime.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod gate;
pub use gate::gate;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
