//! Backend rust framework owner wiring.
//! Wiring owner framework backend rust.

#[path = "rust.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    ACTIX_WEB, AXUM,
};
