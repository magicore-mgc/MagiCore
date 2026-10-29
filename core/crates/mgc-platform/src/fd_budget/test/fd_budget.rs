//! The budget is bounded on both sides and honours an explicit override.
//! Ngân sách bị chặn hai phía và tôn trọng override tường minh.

use super::super::descriptor_budget;

/// Serialises every case that reads or writes the process-wide variable.
///
/// `MAGICORE_MAX_OPEN_FILES` lives in one process-wide map, and the test harness
/// runs cases in parallel threads. Without this lock a case that sets the
/// variable makes the neighbouring case read `77` instead of the computed
/// default, which is exactly the intermittent failure this file used to have.
/// (`MAGICORE_MAX_OPEN_FILES` nằm trong một map toàn process, harness chạy song
/// song; thiếu lock thì case set biến làm case bên cạnh đọc `77` thay vì mặc
/// định — đúng lỗi chập chờn file này từng có.)
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
#[allow(unsafe_code)]
fn the_platform_default_is_bounded_without_an_override() {
    let _guard = lock();
    // Without `MAGICORE_MAX_OPEN_FILES` the budget must still be a usable,
    // bounded number on every OS: Unix derives it from RLIMIT_NOFILE, Windows
    // has no per-process descriptor rlimit and falls back to the default.
    // (Không có override, ngân sách vẫn phải dùng được và bị chặn hai phía trên
    // mọi OS: Unix lấy từ RLIMIT_NOFILE, Windows không có rlimit nên dùng mặc định.)
    // SAFETY: the guard above makes this test the only reader/writer of the
    // variable; removing it restores the platform default.
    // AN TOÀN: guard ở trên bảo đảm test này là chủ biến; xoá biến trả về mặc
    // định nền tảng.
    unsafe { std::env::remove_var("MAGICORE_MAX_OPEN_FILES") };
    let budget = descriptor_budget();
    assert!(
        (32..=512).contains(&budget),
        "platform default budget must stay bounded on every OS: {budget}"
    );
}

#[test]
fn the_budget_is_a_fraction_of_the_process_limit_within_bounds() {
    let _guard = lock();
    let budget = descriptor_budget();
    assert!(budget >= 32, "budget must not collapse: {budget}");
    assert!(
        budget <= 512,
        "budget must not exceed the ceiling: {budget}"
    );
}

#[test]
#[allow(unsafe_code)]
fn an_explicit_override_is_honoured() {
    let _guard = lock();
    // The variable is process-wide, so this test owns it for its duration.
    // Biến là toàn process, nên test này sở hữu nó trong thời gian chạy.
    // SAFETY: edition 2024 requires unsafe for set_var; no other thread reads
    // this variable in a way that could race a test run.
    // AN TOÀN: edition 2024 buộc unsafe cho set_var; không thread nào khác đọc
    // biến này theo cách có thể đua với lần chạy test.
    unsafe { std::env::set_var("MAGICORE_MAX_OPEN_FILES", "77") };
    assert_eq!(descriptor_budget(), 77);
    unsafe { std::env::remove_var("MAGICORE_MAX_OPEN_FILES") };
}

#[test]
#[allow(unsafe_code)]
fn a_zero_override_is_ignored() {
    let _guard = lock();
    // Zero would deadlock the semaphore; it must fall back, not be taken
    // literally.
    // Zero sẽ làm semaphore kẹt; phải quay về mặc định, không lấy đúng nghĩa.
    unsafe { std::env::set_var("MAGICORE_MAX_OPEN_FILES", "0") };
    assert!(descriptor_budget() >= 32);
    unsafe { std::env::remove_var("MAGICORE_MAX_OPEN_FILES") };
}
