// Proxy environment tests — preserve env precedence and defaults.
// Test môi trường proxy — giữ ưu tiên env và giá trị mặc định.

#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::field_reassign_with_default)]

use super::*;
use crate::proxy::test_support::ProxyEnvGuard;

#[test]

fn no_proxy_defaults() {
    // Ambient proxy env (set by the invoking shell) must not leak in.
    // (Env proxy ambient do shell set không được lọt vào.)
    let _guard = ProxyEnvGuard::new();
    let cfg = ProxyConfig::from_env();
    assert!(cfg.no_proxy.contains(&"localhost".into()));
    assert!(cfg.no_proxy.contains(&"127.0.0.1".into()));
}

#[test]
#[allow(unsafe_code)]
fn no_proxy_env_overrides_defaults() {
    // Gate 11-B.2 aftermath: an explicitly SET NO_PROXY is the authority —
    // from_env() must return exactly that list and must NOT merge the
    // localhost defaults into it.
    // (Gate 11-B.2 aftermath: NO_PROXY được set RÕ là thẩm quyền — from_env()
    // phải trả đúng list đó và KHÔNG được trộn default localhost vào.)
    let _guard = ProxyEnvGuard::new();
    // SAFETY: inside the serialised `env_lock` section owned by `_guard`,
    // restored on drop. (SAFETY: nằm trong đoạn `env_lock` tuần tự hoá do
    // `_guard` giữ, được khôi phục khi drop.)
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { env::set_var("NO_PROXY", "internal.example") };

    let cfg = ProxyConfig::from_env();
    assert!(cfg.no_proxy.contains(&"internal.example".into()));
    // Defaults are absent when the env overrides them.
    // Giá trị mặc định không còn khi biến môi trường ghi đè.
    // (Default biến mất khi env ghi đè.)
    assert!(!cfg.no_proxy.contains(&"localhost".into()));
    assert!(!cfg.no_proxy.contains(&"127.0.0.1".into()));
    assert!(cfg.is_bypassed("http://internal.example"));
    assert!(!cfg.is_bypassed("http://localhost:4315"));
}
