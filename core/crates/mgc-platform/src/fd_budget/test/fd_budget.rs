//! The budget is bounded on both sides and honours an explicit override.
//! Ngân sách bị chặn hai phía và tôn trọng override tường minh.

use super::super::descriptor_budget;

#[test]
fn the_budget_is_a_fraction_of_the_process_limit_within_bounds() {
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
    // Zero would deadlock the semaphore; it must fall back, not be taken
    // literally.
    // Zero sẽ làm semaphore kẹt; phải quay về mặc định, không lấy đúng nghĩa.
    unsafe { std::env::set_var("MAGICORE_MAX_OPEN_FILES", "0") };
    assert!(descriptor_budget() >= 32);
    unsafe { std::env::remove_var("MAGICORE_MAX_OPEN_FILES") };
}
