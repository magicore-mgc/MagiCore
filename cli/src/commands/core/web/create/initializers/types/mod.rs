//! Initializer data contract owner.
//! Owner contract dữ liệu initializer.

#[path = "types.rs"]
mod implementation;

pub(in crate::commands::core::web::implementation) use implementation::{
    ArgSlot, InitializerRecord, Layout, Mode, Runtime, VersionPolicy,
};
