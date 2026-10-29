//! Wires the install-hook guard and its nearest tests.
//! Nối cổng hook cài đặt và test gần nhất.

mod install_hook_guard;
pub use install_hook_guard::{ALLOW_INSTALL_HOOKS_ENV, InstallHookGuard};
#[cfg(test)]
mod test;
