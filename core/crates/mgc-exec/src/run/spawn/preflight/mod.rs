//! Pre-spawn validation and dry-run owner facade.
//! Facade owner xác thực trước spawn và dry-run.

#[path = "preflight.rs"]
mod implementation;
#[path = "types/mod.rs"]
mod types;

pub(super) use implementation::{complete, prepare};
pub(super) use types::PreparedInvocation;
