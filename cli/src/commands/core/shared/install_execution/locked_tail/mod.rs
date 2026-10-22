//! Locked install tail wiring.
//! Wiring tail install đã khóa.

use super::super::*;

#[path = "locked_tail.rs"]
mod implementation;

pub(in crate::commands::core::shared::implementation::implementation) use implementation::install_with_adapter_locked;
