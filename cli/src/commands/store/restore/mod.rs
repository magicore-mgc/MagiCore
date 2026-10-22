//! Wires validation, preservation and restore owners for project Web-store state.
//! Nối các owner xác thực, bảo toàn và khôi phục Web-store theo project.

mod backup_validation;
mod database_restore;
mod error;
mod preserve_state;
mod project_state;
mod restore;
mod safety_destination;
mod virtual_store_restore;

pub(super) use restore::run_restore;
