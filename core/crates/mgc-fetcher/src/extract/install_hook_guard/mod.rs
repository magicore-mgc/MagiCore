//! Wires the install-hook guard and its nearest tests.
//! Nối cổng hook cài đặt và test gần nhất.

mod install_hook_guard;
pub use install_hook_guard::{ALLOW_INSTALL_HOOKS_ENV, InstallHookGuard};
pub(in crate::extract) use install_hook_guard::classify;
#[cfg(test)]
mod test;
