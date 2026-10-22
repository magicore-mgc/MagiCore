//! Fullstack php framework owner wiring.
//! Wiring owner framework fullstack php.

#[path = "php.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    REACT_SYMFONY, VUE_LARAVEL, VUE_SYMFONY,
};
