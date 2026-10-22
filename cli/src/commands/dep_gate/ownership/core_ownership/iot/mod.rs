//! iot ownership policy wiring — wiring policy ownership iot.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod iot;
pub(super) use iot::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
