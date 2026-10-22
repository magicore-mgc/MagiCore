//! Post-add install and recovery wiring.
//! Wiring install và recovery sau add.

#[path = "post_add_install.rs"]
mod implementation;

pub(crate) use implementation::finish_post_add;
