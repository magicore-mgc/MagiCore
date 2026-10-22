//! Web lockfile dependency explanation facade.
//! Facade giải thích phụ thuộc từ lockfile Web.

#[path = "error.rs"]
mod error;
#[path = "why.rs"]
mod implementation;

pub use implementation::why;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
