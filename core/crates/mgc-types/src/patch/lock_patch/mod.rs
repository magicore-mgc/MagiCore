//! Lockfile patch-record exports and nearest tests.
//! Export record patch lockfile và test gần owner.

mod lock_patch;

pub use self::lock_patch::LockPatch;

#[cfg(test)]
#[path = "test/lock_patch.rs"]
mod tests;
