//! Backend php framework owner wiring.
//! Wiring owner framework backend php.

#[path = "php.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    LARAVEL, SYMFONY,
};
