//! lib package dispatch facade. / Facade dispatch package của core lib.

mod lib;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::dispatch) use lib::{dispatch, matches};
