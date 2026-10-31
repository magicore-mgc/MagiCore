//! Core initializer dispatch owner.
//! Owner điều phối initializer core.

#[path = "core_initializer.rs"]
mod implementation;

pub(crate) use implementation::record_for;
pub(crate) use implementation::try_run;
