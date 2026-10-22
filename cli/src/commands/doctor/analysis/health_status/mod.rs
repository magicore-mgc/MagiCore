//! Health label derivation.
//! Tính health label.

mod health_status;

pub(super) use health_status::classify;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
