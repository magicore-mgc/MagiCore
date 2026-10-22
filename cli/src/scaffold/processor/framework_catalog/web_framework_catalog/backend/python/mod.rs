//! Backend python framework owner wiring.
//! Wiring owner framework backend python.

#[path = "python.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    DJANGO, FASTAPI, FLASK,
};
