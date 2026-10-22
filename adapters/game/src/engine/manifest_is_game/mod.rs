//! Wire project identity checks for supported game engines.
//! Nối kiểm tra identity project cho các game engine được hỗ trợ.

mod manifest_is_game;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

pub(crate) use manifest_is_game::manifest_is_game;
