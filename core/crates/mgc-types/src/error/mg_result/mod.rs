//! Shared result alias exports and nearest tests.
//! Tái xuất alias result dùng chung và test gần owner.

mod mg_result;

pub use self::mg_result::MgResult;

#[cfg(test)]
#[path = "test/mg_result.rs"]
mod tests;
