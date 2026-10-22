//! KeyPair owner exports — Export owner KeyPair.
//! Named siblings own implementation and local tests — Sibling có tên sở hữu code và test.

mod key_pair;
pub use key_pair::KeyPair;

#[cfg(test)]
#[path = "test/key_pair.rs"]
mod tests;
