//! app ownership policy wiring — wiring policy ownership app.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod app;
pub(super) use app::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
