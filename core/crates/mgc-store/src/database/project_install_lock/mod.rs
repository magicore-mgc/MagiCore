//! Project install lock owner wiring and public type export.
//! Wiring owner khóa install project và export type công khai.
mod project_install_lock;
#[path = "project_install_lock_handle/mod.rs"]
mod project_install_lock_handle;
pub use project_install_lock_handle::ProjectInstallLock;
