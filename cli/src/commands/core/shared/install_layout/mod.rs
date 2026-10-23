//! Install-layout policy owner (shared by every core).
//! Owner policy layout install (dùng chung mọi core).

mod install_layout;

pub use install_layout::should_use_legacy_flat_layout;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
