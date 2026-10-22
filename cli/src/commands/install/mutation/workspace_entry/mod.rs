//! Mixed-core workspace entry into the generic per-project install transaction.
//! Điểm vào workspace đa core tới transaction install từng project.

mod workspace_entry;

pub(crate) use workspace_entry::install_into_root_ws;
