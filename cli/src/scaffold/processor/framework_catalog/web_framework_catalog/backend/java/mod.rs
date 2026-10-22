//! Backend java framework owner wiring.
//! Wiring owner framework backend java.

#[path = "java.rs"]
mod implementation;

pub(in crate::scaffold::processor::framework_catalog::web_framework_catalog) use implementation::{
    QUARKUS, SPRING_BOOT,
};
