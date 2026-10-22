//! Wires the lock write owner.
//! Wiring owner lock write.

mod atomic_publication;
mod lock_write;
mod v4_lane_write;

#[cfg(test)]
pub(crate) use lock_write::write_canonical_lock;
pub(crate) use lock_write::{direct_package_ids, write_canonical_lock_with_roots};

#[cfg(test)]
pub(in crate::install) use atomic_publication::atomic_write_canonical_lock;
