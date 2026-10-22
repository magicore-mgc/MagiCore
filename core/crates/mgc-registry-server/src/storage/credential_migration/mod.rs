//! Atomic replacement of the legacy plaintext credential table.
//! Thay thế nguyên tử bảng credential plaintext cũ.

mod credential_migration;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
