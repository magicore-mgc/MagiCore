//! Project install guard owner.
//! Owner khoá install theo project.

#[path = "project_install_guard.rs"]
mod implementation;

pub(crate) use implementation::acquire;
