//! Lock-reuse owner wiring.
//! Wiring owner tái sử dụng lock.

use super::*;
#[path = "lock_reuse.rs"]
mod implementation;
pub(in crate::commands::core::shared::implementation::implementation) use implementation::try_install_added_packages_from_lock;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
