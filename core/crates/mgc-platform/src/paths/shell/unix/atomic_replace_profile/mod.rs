//! Atomic Unix profile persistence owner.
//! Owner lưu profile Unix theo cách thay thế nguyên tử.

mod atomic_replace_profile;

pub(super) use atomic_replace_profile::atomic_replace_profile;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
