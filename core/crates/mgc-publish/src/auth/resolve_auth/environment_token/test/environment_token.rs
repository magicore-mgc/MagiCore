//! Environment token source test / test nguồn token environment.

#![allow(unsafe_code)]

use std::ffi::OsString;

use super::super::resolve_environment_token;

struct RestoreEnvironment {
    mgc: Option<OsString>,
    npm: Option<OsString>,
}

impl RestoreEnvironment {
    fn capture() -> Self {
        Self {
            mgc: std::env::var_os("MGC_NPM_TOKEN"),
            npm: std::env::var_os("NPM_TOKEN"),
        }
    }
}

impl Drop for RestoreEnvironment {
    fn drop(&mut self) {
        // SAFETY: test-only operation on process-local state / FFI with
        // caller-owned buffers kept alive for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
        // buffer do caller sở hữu và còn sống trong lời gọi.
        unsafe {
            match self.mgc.take() {
                Some(value) => std::env::set_var("MGC_NPM_TOKEN", value),
                None => std::env::remove_var("MGC_NPM_TOKEN"),
            }
            match self.npm.take() {
                Some(value) => std::env::set_var("NPM_TOKEN", value),
                None => std::env::remove_var("NPM_TOKEN"),
            }
        }
    }
}

#[test]
fn mgc_token_precedes_npm_token() {
    let _restore = RestoreEnvironment::capture();
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe {
        std::env::set_var("MGC_NPM_TOKEN", "mgc-token");
        std::env::set_var("NPM_TOKEN", "npm-token");
    }
    let auth = resolve_environment_token().expect("environment token should resolve");
    assert_eq!(auth.token.as_deref(), Some("mgc-token"));
}
