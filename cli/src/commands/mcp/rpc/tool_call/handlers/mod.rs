//! MCP tool handler collection wiring.
//! Khai báo handler theo tool và export các hàm nội bộ cho dispatcher.

mod add;
mod audit;
mod config;
mod handlers;
mod install;
mod outdated;
mod uninstall;
mod versions;
mod workspace_info;

pub(super) use add::handle_add;
pub(super) use audit::handle_audit;
pub(super) use config::handle_config;
pub(super) use handlers::dispatch;
pub(super) use install::handle_install;
pub(super) use outdated::handle_outdated;
pub(super) use uninstall::handle_uninstall;
pub(super) use versions::handle_versions;
pub(super) use workspace_info::handle_workspace_info;
