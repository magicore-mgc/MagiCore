//! Pipeline layout owner — wiring only.
//! Owner layout pipeline — chỉ wiring.

#[path = "layout.rs"]
mod layout;

pub use layout::{GENERATED_PROVIDERS, PipelineFile, can_generate, generation_plan};
