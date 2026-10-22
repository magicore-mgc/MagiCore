//! Fullstack python framework owner wiring.
//! Wiring owner framework fullstack python.

#[path = "python.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    REACT_DJANGO, REACT_FASTAPI, REACT_FLASK, SVELTE_FASTAPI, VUE_DJANGO, VUE_FASTAPI, VUE_FLASK,
};
