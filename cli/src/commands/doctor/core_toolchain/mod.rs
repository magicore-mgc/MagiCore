//! Core-to-toolchain requirement mapping.
//! Ánh xạ yêu cầu toolchain theo core.

mod core_toolchain;

pub(super) use core_toolchain::requirements_for_core;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
