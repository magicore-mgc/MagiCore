//! Fullstack special framework owner wiring.
//! Wiring owner framework fullstack special.

#[path = "special.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    CUSTOM, REMIX,
};
