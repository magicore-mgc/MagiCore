//! iot package dispatch facade. / Facade dispatch package của core iot.

mod iot;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::dispatch) use iot::{dispatch, matches};
