//! Standalone install gateway wiring.
//! Wiring gateway install độc lập.

use super::super::*;

#[path = "gateway_entry.rs"]
mod implementation;

pub(crate) use implementation::install_with_adapter;
