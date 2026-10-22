//! Per-project transactional install owner.
//! Owner giao dịch install cho từng project.

mod mutation;
mod offline_guard;
mod workspace_entry;

pub(super) use mutation::install_into_root;
pub(crate) use workspace_entry::install_into_root_ws;
