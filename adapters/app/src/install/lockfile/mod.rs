//! Lockfile owner for App dependency installs.
//! Module chủ quản lockfile cho install dependency App.
mod atomic_publication;
mod direct_roots;
mod error;
mod lock_write;
mod owner_core;
mod v3_merge;
mod v4_merge;

#[cfg(test)]
pub(crate) use atomic_publication::atomic_write_canonical_lock;
pub(crate) use direct_roots::direct_package_ids;
#[cfg(test)]
pub(crate) use lock_write::write_canonical_lock;
pub(crate) use lock_write::write_canonical_lock_with_roots;
#[cfg(test)]
pub(crate) mod test;
