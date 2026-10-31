#[path = "dev_output.rs"]
mod implementation;

pub(in crate::commands::core::web::implementation) use implementation::{
    BACKEND_ROLE, backend_starting_line, readiness_line,
};
