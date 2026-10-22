//! Fullstack rust framework owner wiring.
//! Wiring owner framework fullstack rust.

#[path = "rust.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    REACT_ACTIX_WEB, REACT_AXUM, SVELTE_AXUM, VUE_ACTIX_WEB, VUE_AXUM,
};
