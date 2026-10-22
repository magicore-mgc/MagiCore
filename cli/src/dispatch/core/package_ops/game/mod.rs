//! game package dispatch facade. / Facade dispatch package của core game.

mod game;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::dispatch) use game::{dispatch, matches};
