//! Writes workspace membership and cross-core edges in canonical order.
//! File này chỉ nối owner workspace_section.

mod workspace_section;

pub(crate) use workspace_section::append_workspace_section;
