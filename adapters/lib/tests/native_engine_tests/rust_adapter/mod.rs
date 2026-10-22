//! Rust adapter test owner facade.
//! Facade owner test rust adapter theo hành vi.

#[path = "rust_adapter.rs"]
mod implementation;

pub(crate) use implementation::lib_adapter_resolve_uses_native_engine_via_env;
