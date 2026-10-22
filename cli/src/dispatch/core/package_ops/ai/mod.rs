//! ai package dispatch facade. / Facade dispatch package của core ai.

mod ai;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::dispatch) use ai::{dispatch, matches};
