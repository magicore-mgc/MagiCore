//! Engine choice owner.
//! Owner chọn engine.

#[path = "engine_choice.rs"]
mod implementation;

pub(crate) use implementation::{Engine, Preference, choose};
