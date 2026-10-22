//! web package dispatch facade. / Facade dispatch package của core web.

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
mod web;

pub(in crate::dispatch) use web::{dispatch, matches};
