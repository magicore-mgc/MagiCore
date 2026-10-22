//! ai ownership policy wiring — wiring policy ownership ai.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod ai;
pub(super) use ai::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
