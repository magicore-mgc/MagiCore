//! Wires v4 rewrite policy, atomic publication, and local write errors.
//! Nối policy rewrite v4, publish atomic và lỗi ghi cục bộ.

mod error;
mod v4_write;
pub use self::v4_write::{atomic_write_v4_bytes, ensure_v4_unsigned_for_rewrite};
