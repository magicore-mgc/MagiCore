//! Install preparation wiring.
//! Wiring bước chuẩn bị install.

use super::super::*;

#[path = "preparation.rs"]
mod implementation;

pub(crate) use implementation::prepare_install_execution;
