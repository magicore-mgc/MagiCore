// Shared proxy-test support — serialize and restore process environment.
// Hỗ trợ test proxy dùng chung — khóa và phục hồi môi trường tiến trình.

use std::env;
use std::sync::{Mutex, MutexGuard, OnceLock};

/// Serialize env mutation across tests: env is process-global while cargo
/// runs tests on parallel threads, so unguarded `set_var`/`remove_var`
/// would race the other proxy tests in this binary.
/// (Tuần tự hoá mutation env giữa các test: env là global của process trong
/// khi cargo chạy test song song nhiều thread — set/remove không khoá sẽ
/// đua với các test proxy khác trong cùng binary.)
fn env_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The 6 proxy env vars read by `ProxyConfig::from_env()` — all of them must
/// be backed up and restored, or an ambient shell NO_PROXY (even an EMPTY
/// one) flips the defaults tests.
/// (6 biến env proxy mà `from_env()` đọc — phải backup/khôi phục đủ cả 6,
/// nếu không NO_PROXY ambient của shell — kể cả RỖNG — sẽ lật kết quả các
/// test mặc định.)
const PROXY_ENV_VARS: [&str; 6] = [
    "HTTP_PROXY",
    "http_proxy",
    "HTTPS_PROXY",
    "https_proxy",
    "NO_PROXY",
    "no_proxy",
];

/// RAII guard: backs up all 6 proxy env vars, removes them for the duration
/// of the test, and restores the exact original state on Drop — even when
/// the test panics. Holding `env_lock` for its whole lifetime serialises
/// the env-touching tests in this binary.
/// (RAII guard: backup đủ 6 biến env proxy, xoá trong lúc test và khôi phục
/// NGUYÊN TRẠNG ban đầu khi Drop — kể cả khi test panic. Giữ `env_lock`
/// suốt vòng đời để tuần tự hoá các test đụng env trong binary này.)
pub(super) struct ProxyEnvGuard {
    _lock: MutexGuard<'static, ()>,
    saved: Vec<(&'static str, Option<String>)>,
}

impl ProxyEnvGuard {
    #[allow(unsafe_code)]
    pub(super) fn new() -> Self {
        let lock = env_lock();
        let saved: Vec<_> = PROXY_ENV_VARS
            .iter()
            .map(|&key| (key, env::var(key).ok()))
            .collect();
        for (key, _) in &saved {
            // SAFETY: env mutation is serialised process-wide by `env_lock`
            // held in `self._lock`; each var is restored to its captured
            // state in `Drop`. (SAFETY: mutation env được tuần tự hoá toàn
            // process bởi `env_lock` giữ trong `self._lock`; mỗi biến được
            // khôi phục về trạng thái đã capture trong `Drop`.)
            unsafe { env::remove_var(key) };
        }
        Self { _lock: lock, saved }
    }
}

impl Drop for ProxyEnvGuard {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        for (key, value) in &self.saved {
            // SAFETY: still inside the `env_lock` section opened in `new`
            // (guard field outlives this drop); restores the captured state.
            // (SAFETY: vẫn trong đoạn `env_lock` mở ở `new` (field guard sống
            // dài hơn drop này); khôi phục đúng trạng thái đã capture.)
            unsafe {
                match value {
                    Some(v) => env::set_var(key, v),
                    None => env::remove_var(key),
                }
            }
        }
    }
}
