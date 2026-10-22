//! web ownership policy wiring — wiring policy ownership web.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod web;
pub(super) use web::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
