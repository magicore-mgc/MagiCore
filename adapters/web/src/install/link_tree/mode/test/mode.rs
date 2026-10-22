//! Verify strict-link environment aliases and safe defaults.
//! Kiểm tra alias environment strict-link và default an toàn.

#![allow(unsafe_code)]

use std::ffi::OsString;
use std::sync::MutexGuard;

use super::super::implementation::{StrictTreeLinkMode, strict_tree_link_mode};

struct EnvGuard {
    key: &'static str,
    previous: Option<OsString>,
    _lock: MutexGuard<'static, ()>,
}

impl EnvGuard {
    fn new(key: &'static str) -> Self {
        let lock = crate::environment_test_lock()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        Self {
            key,
            previous: std::env::var_os(key),
            _lock: lock,
        }
    }

    fn set(&self, value: Option<&str>) {
        unsafe {
            match value {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match self.previous.take() {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

#[test]
fn strict_mode_aliases_and_unknown_values_keep_their_modes() {
    let env = EnvGuard::new("MAGICORE_WEB_STRICT_TREE_MODE");

    for alias in ["hardlink", "copy", "compat"] {
        env.set(Some(alias));
        assert_eq!(strict_tree_link_mode(), StrictTreeLinkMode::Hardlink);
    }
    for alias in ["symlink", "link", "fast", "  FAST  "] {
        env.set(Some(alias));
        assert_eq!(strict_tree_link_mode(), StrictTreeLinkMode::Symlink);
    }
    env.set(Some("unknown"));
    assert_eq!(strict_tree_link_mode(), StrictTreeLinkMode::Hardlink);
    env.set(None);
    assert_eq!(strict_tree_link_mode(), StrictTreeLinkMode::Hardlink);
}
