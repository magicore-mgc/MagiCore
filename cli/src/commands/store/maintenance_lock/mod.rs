//! Shared project lock owner for Web store maintenance operations.
//! Owner khóa project dùng chung cho thao tác bảo trì Web store.

mod error;
mod maintenance_lock;
pub(in crate::commands) use error::MaintenanceLockError;
pub(in crate::commands) use maintenance_lock::acquire_maintenance_lock;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
