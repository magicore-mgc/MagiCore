//! cloud ownership policy wiring — wiring policy ownership cloud.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod cloud;
pub(super) use cloud::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
