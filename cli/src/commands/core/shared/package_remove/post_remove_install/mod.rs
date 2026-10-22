//! Post-remove install and journal completion wiring.
//! Wiring install và hoàn tất journal sau remove.

#[path = "post_remove_install.rs"]
mod implementation;

pub(crate) use implementation::{finish_no_removal, finish_remove};
