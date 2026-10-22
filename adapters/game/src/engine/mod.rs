//! Game engine facade with stable public exports.
//! Facade engine game giữ các public export ổn định.

mod detect_engine;
mod engine;
mod manifest_is_game;

pub use detect_engine::detect_engine;
pub use engine::GameEngine;
pub(crate) use manifest_is_game::manifest_is_game;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
