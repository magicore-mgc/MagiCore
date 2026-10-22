//! Lock writer coordinator wiring / Wiring điều phối bộ ghi lock.
mod lock_write;
#[cfg(test)]
pub(crate) use lock_write::write_canonical_lock;
pub(crate) use lock_write::write_canonical_lock_with_roots;
