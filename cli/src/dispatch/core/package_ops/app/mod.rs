//! app package dispatch facade. / Facade dispatch package của core app.

mod app;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::dispatch) use app::{dispatch, matches};
