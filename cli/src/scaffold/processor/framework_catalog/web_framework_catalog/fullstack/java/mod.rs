//! Fullstack java framework owner wiring.
//! Wiring owner framework fullstack java.

#[path = "java.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    REACT_QUARKUS, REACT_SPRING, SVELTE_QUARKUS, VUE_QUARKUS,
};
