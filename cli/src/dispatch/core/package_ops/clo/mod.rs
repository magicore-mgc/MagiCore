//! clo package dispatch facade. / Facade dispatch package của core clo.

mod clo;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::dispatch) use clo::{dispatch, matches};
