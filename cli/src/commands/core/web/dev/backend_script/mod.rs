#[path = "backend_script.rs"]
mod implementation;

pub(in crate::commands::core::web::implementation) use implementation::{
    BackendScript, is_backend_half, resolve_backend_script,
};
