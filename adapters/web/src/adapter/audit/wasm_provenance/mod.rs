//! WASM-provenance wiring / wiring provenance WASM.
mod wasm_provenance;
pub(in crate::adapter) use wasm_provenance::{audit_wasm_provenance, find_wasm_artifacts};
