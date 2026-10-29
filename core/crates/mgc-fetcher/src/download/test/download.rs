//! SEC-9 download cap resolution tests.
//! Test xác định giới hạn tải SEC-9.

use super::super::{
    DEFAULT_MAX_DOWNLOAD_BYTES, MAX_DOWNLOAD_BYTES_ENV, max_download_bytes,
};
use std::sync::Mutex;

/// The environment is process-global; serialize tests that mutate it.
/// Environment là toàn cục process; tuần tự hóa test có thay đổi biến.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn set_override(raw: Option<&str>) {
    match raw {
        Some(raw) => {
            // SAFETY: test-only mutation of process-local state; `ENV_LOCK` is
            // held so no sibling test thread reads the environment concurrently.
            // AN TOÀN: chỉ test ghi state cục bộ process; `ENV_LOCK` đang giữ
            // nên thread test khác không đọc env đồng thời.
            #[allow(unsafe_code)]
            unsafe {
                std::env::set_var(MAX_DOWNLOAD_BYTES_ENV, raw)
            }
        }
        None => {
            // SAFETY: test-only mutation of process-local state under `ENV_LOCK`.
            // AN TOÀN: chỉ test ghi state cục bộ process dưới `ENV_LOCK`.
            #[allow(unsafe_code)]
            unsafe {
                std::env::remove_var(MAX_DOWNLOAD_BYTES_ENV)
            }
        }
    }
}

fn with_override<T>(raw: Option<&str>, body: impl FnOnce() -> T) -> T {
    let _guard = ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let previous = std::env::var_os(MAX_DOWNLOAD_BYTES_ENV);
    set_override(raw);
    let result = body();
    set_override(previous.as_deref().and_then(|value| value.to_str()));
    result
}

#[test]
fn default_cap_is_the_named_constant() {
    let cap = with_override(None, max_download_bytes);
    assert_eq!(cap, DEFAULT_MAX_DOWNLOAD_BYTES);
    assert!(cap > 0);
}

#[test]
fn valid_override_replaces_the_default() {
    let cap = with_override(Some("2048"), max_download_bytes);
    assert_eq!(cap, 2048);
}

#[test]
fn zero_override_falls_back_to_the_default_cap() {
    let cap = with_override(Some("0"), max_download_bytes);
    assert_eq!(cap, DEFAULT_MAX_DOWNLOAD_BYTES);
}

#[test]
fn non_numeric_override_falls_back_to_the_default_cap() {
    let cap = with_override(Some("not-a-number"), max_download_bytes);
    assert_eq!(cap, DEFAULT_MAX_DOWNLOAD_BYTES);
}
