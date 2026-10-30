//! Project-name owner exports.
//! Export owner tên dự án.

mod project_name;

pub(in crate::commands::init) use project_name::{ask_project_name, resolve_non_interactive_name};
