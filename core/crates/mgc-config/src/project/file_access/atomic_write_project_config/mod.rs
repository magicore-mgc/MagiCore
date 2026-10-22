//! Atomic project-config writer wiring.
//! Wiring bộ ghi cấu hình project atomic.
mod atomic_write_project_config;
pub(in crate::project::project) use atomic_write_project_config::atomic_write_project_config;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
