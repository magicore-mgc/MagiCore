//! hardware package dispatch facade. / Facade dispatch package của core hardware.

mod hardware;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::dispatch) use hardware::{dispatch, matches};
