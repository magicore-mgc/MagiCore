//! Fullstack go framework owner wiring.
//! Wiring owner framework fullstack go.

#[path = "go.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    REACT_ECHO, REACT_FIBER, REACT_GIN, SVELTE_GIN, VUE_ECHO, VUE_FIBER, VUE_GIN,
};
