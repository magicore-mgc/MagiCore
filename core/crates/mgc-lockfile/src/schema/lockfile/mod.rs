//! Wires the v3 lockfile root and its local contract tests.
//! Nối root lockfile v3 và test contract gần owner.

mod lockfile;
pub use self::lockfile::{Lockfile, LockfileMetadata, SignerInfo};
#[cfg(test)]
mod test;
