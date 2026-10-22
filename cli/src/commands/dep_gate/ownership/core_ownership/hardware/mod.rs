//! hardware ownership policy wiring — wiring policy ownership hardware.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod hardware;
pub(super) use hardware::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
