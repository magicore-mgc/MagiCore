//! Backing File owner wiring.
//! Wiring owner backing file.

#[path = "backing_file.rs"]
mod implementation;

pub use implementation::backing_link_file;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
