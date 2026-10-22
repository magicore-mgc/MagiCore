//! Tests the bounded thread-local deferred close ledger.
//! Kiểm tra ledger deferred close có giới hạn theo thread.

use super::{
    MAX_DEFERRED_HANDLE_CLOSE_DETAILS, record_handle_close_failure,
    take_deferred_handle_close_error,
};

#[test]
fn deferred_handle_close_failures_are_thread_local_and_drained_as_one_error() {
    for code in 1..=MAX_DEFERRED_HANDLE_CLOSE_DETAILS + 2 {
        record_handle_close_failure(code as u32);
    }

    let other_thread_message = std::thread::spawn(|| {
        record_handle_close_failure(99);
        take_deferred_handle_close_error()
            .expect("other thread must retain its own close failure")
            .to_string()
    })
    .join()
    .expect("thread-local ledger check must not panic");
    assert!(other_thread_message.contains("1 Windows handle(s)"));
    assert!(other_thread_message.contains("error code 99"));

    let error = take_deferred_handle_close_error()
        .expect("deferred native close failures must be surfaced");
    let message = error.to_string();
    assert!(message.contains("34 Windows handle(s)"));
    assert!(message.contains("error code 1"));
    assert!(message.contains("error code 32"));
    assert!(message.contains("2 additional failure(s) omitted"));
    assert!(take_deferred_handle_close_error().is_none());
}
