//! Output signing owner.
//! Owner ký output.

#[path = "sign.rs"]
mod implementation;

pub(crate) use implementation::{Signed, sign_artifact};
