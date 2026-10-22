//! Backend go framework owner wiring.
//! Wiring owner framework backend go.

#[path = "go.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    ECHO, FIBER, GIN,
};
