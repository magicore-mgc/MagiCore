//! Core-to-toolchain requirement mapping.
//! Ánh xạ yêu cầu toolchain theo core.

mod core_toolchain;
#[path = "split_server/mod.rs"]
mod split_server;

pub(super) use core_toolchain::{CoreToolRequirement, requirements_for_project};

#[cfg(test)]
pub(super) use core_toolchain::requirements_for_core;
pub(in crate::commands::doctor::core_toolchain) use split_server::split_server_half_tools;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
