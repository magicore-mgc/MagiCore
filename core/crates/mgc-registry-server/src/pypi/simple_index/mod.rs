//! PyPI simple-index behavior owner.
//! Owner hành vi simple-index PyPI.

mod error;
mod simple_index;
pub(super) use simple_index::simple_index;

#[cfg(test)]
mod test;
