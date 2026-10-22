//! cicd package dispatch facade. / Facade dispatch package của core cicd.

mod cicd;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::dispatch) use cicd::{dispatch, matches};
