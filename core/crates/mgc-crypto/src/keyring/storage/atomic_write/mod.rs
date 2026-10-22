//! Atomic keyring writer wiring — Wiring writer keyring atomic.
//! Wire temp creation, durability and publication — Nối tạo temp, durability và publish.

mod atomic_write;
pub(in crate::keyring) use atomic_write::write_private_file_atomic;

#[cfg(test)]
#[path = "test/atomic_write.rs"]
mod tests;
