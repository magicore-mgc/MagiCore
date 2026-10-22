//! Atomic publication wiring / Wiring publish nguyên tử.
mod atomic_publication;
#[cfg(test)]
pub(crate) use atomic_publication::atomic_write_canonical_lock;
pub(super) use atomic_publication::atomic_write_canonical_lock_typed;
