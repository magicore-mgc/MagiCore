//! `exec` owner facade.
//! Facade owner `exec`.

mod exec;

#[cfg(test)]
#[path = "test/exec.rs"]
mod tests;

pub use exec::*;
