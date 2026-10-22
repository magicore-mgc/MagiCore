//! Wires lock-write failpoint behavior and its contract tests.
//! Nối hành vi failpoint ghi lock và test contract.

mod failpoint;
pub use self::failpoint::{LOCK_FAILPOINTS, lock_failpoint};
