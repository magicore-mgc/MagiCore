//! IoT framework and board catalog wiring.
//! Wiring catalog framework và board IoT.

#[path = "catalog.rs"]
mod implementation;
pub(super) use implementation::{default_board, framework_for_board, supports};
