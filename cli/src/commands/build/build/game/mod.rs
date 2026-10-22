#[path = "error.rs"]
mod error;
use error::GameBuildError;

#[path = "game.rs"]
mod implementation;

pub(super) use implementation::build_game;
