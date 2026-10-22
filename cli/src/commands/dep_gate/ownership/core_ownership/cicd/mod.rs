//! cicd ownership policy wiring — wiring policy ownership cicd.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod cicd;
pub(super) use cicd::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
