//! Game record wiring — nối record Game.
#[path = "game.rs"]
mod implementation;

pub(super) use self::implementation::RECORDS;
