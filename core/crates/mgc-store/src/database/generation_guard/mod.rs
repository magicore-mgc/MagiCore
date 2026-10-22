//! CAS generation guard owner wiring and public type export.
//! Wiring owner guard CAS generation và export type công khai.
#[path = "cas_generation_guard/mod.rs"]
mod cas_generation_guard;
mod generation_guard;
pub use cas_generation_guard::CasGenerationGuard;
