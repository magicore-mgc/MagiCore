//! Managed Unix PATH marker block owner.
//! Owner block marker PATH Unix.

mod managed_path_profile_block;

pub(super) use managed_path_profile_block::{
    managed_path_profile_block, replace_managed_profile_block,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
