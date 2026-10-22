//! Atomic writer wiring / Nối module ghi atomic.
mod atomic_write;
pub(crate) use atomic_write::atomic_write_web_lockfile;
