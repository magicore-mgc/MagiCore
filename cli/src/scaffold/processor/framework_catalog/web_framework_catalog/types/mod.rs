//! Shared Web framework metadata type wiring.
//! Wiring type metadata framework Web dùng chung.

#[path = "types.rs"]
mod implementation;

pub(in crate::scaffold::processor) use implementation::WebFrameworkConfig;
