//! Wires preservation of current project store state.
//! Nối owner bảo toàn state store hiện tại của project.

mod error;
mod preserve_state;

pub(super) use error::PreserveStateError;
pub(super) use preserve_state::preserve_current_state;
