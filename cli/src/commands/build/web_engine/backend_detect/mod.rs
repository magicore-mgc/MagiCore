//! Backend detection owner.
//! Owner phát hiện backend.

#[path = "backend_detect.rs"]
mod implementation;

pub(crate) use implementation::is_backend_project;
