//! game ownership policy wiring — wiring policy ownership game.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod game;
pub(super) use game::owner_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
