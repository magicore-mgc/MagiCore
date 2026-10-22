//! Wires ownership ledger parsing and local contract tests.
//! Nối parse ledger ownership và test contract gần nhất.

mod ownership_ledger;
pub use self::ownership_ledger::{OwnershipEntry, load_ownership_ledger};
#[cfg(test)]
mod test;
