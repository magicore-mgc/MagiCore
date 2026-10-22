//! AI adapter facade — export the AI package adapter contract.
//! Facade adapter AI — export hợp đồng adapter package AI.

#[path = "adapter/mod.rs"]
mod implementation;
pub use implementation::{AiAdapter, adapter_for, uses_native_python_lane};
