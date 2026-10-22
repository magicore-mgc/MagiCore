//! lib ownership policy wiring — wiring policy ownership lib.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod lib;
pub(super) use lib::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
