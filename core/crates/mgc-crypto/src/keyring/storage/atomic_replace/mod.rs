//! Atomic replacement platform facade — Facade replace atomic theo nền tảng.
//! Preserve native replacement and synchronous errors — Giữ replace native và lỗi đồng bộ.

mod atomic_replace;
pub(super) use atomic_replace::atomic_replace_file;

#[cfg(test)]
#[path = "test/atomic_replace.rs"]
mod tests;
