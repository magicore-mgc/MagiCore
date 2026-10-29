//! Wires the interpreter install-hook detector and its nearest tests.
//! Nối detector hook cài đặt interpreter và test gần nhất.

mod install_hook;
pub use install_hook::{
    ALLOW_INSTALL_HOOKS_ENV, FORBIDDEN_MODE_BITS, InstallHook, InstallHookKind, InstallHookPolicy,
    classify_entry, hooks_allowed_for, reject_install_hooks, warn_hook_escape_hatch,
};
#[cfg(test)]
mod test;
