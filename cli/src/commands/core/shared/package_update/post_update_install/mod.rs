//! Post-update reporting and install wiring.
//! Wiring output và install sau update.

#[path = "post_update_install.rs"]
mod implementation;

pub(crate) use implementation::finish_update;
