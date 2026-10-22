//! Directory Sync owner wiring.
//! Nối owner directory sync.

mod directory_sync;

pub(in crate::cas::store) use self::directory_sync::sync_parent_directories;

#[cfg(test)]
mod test;
