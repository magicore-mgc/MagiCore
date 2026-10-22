//! Canonical temporary path, socket, and environment fixtures for metadata tests.
//! Fixture path tạm canonical, socket và biến môi trường cho test metadata.

use std::io::ErrorKind;
use std::sync::Mutex;
use tokio::net::TcpListener;

pub(super) fn tempdir_real() -> std::io::Result<tempfile::TempDir> {
    let canonical = std::env::temp_dir().canonicalize()?;
    tempfile::tempdir_in(canonical)
}

pub(super) async fn bind_test_listener() -> Option<TcpListener> {
    match TcpListener::bind("127.0.0.1:0").await {
        Ok(listener) => Some(listener),
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            eprintln!("skipping socket-backed test in sandbox: {err}");
            None
        }
        Err(err) => panic!("failed to bind socket-backed test listener: {err}"),
    }
}

pub(super) fn restore_env_var(key: &str, previous: Option<std::ffi::OsString>) {
    if let Some(value) = previous {
        unsafe { std::env::set_var(key, value) };
    } else {
        unsafe { std::env::remove_var(key) };
    }
}

pub(super) fn env_test_lock() -> &'static Mutex<()> {
    crate::environment_test_lock()
}

pub(super) fn reg_key(url: &str) -> String {
    url.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}
