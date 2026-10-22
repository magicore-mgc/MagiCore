//! Hook file merge exports.
//! Export việc gộp file hook.

mod merge;

pub(super) use merge::load_merged;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
