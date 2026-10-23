//! Django starter owner.
//! Owner starter Django.

mod django;

pub(crate) use django::write_django;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
