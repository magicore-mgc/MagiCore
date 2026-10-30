//! Bounded per-thread ledger for failures from `CloseHandle` in destructors.
//! Ledger hữu hạn theo thread cho lỗi `CloseHandle` phát sinh trong destructor.

#[cfg(windows)]
use super::super::super::error::{WindowsApiFailure, WindowsJobError};
#[cfg(windows)]
use std::cell::RefCell;

#[cfg(windows)]
const MAX_DEFERRED_HANDLE_CLOSE_DETAILS: usize = 32;

#[cfg(windows)]
#[derive(Default)]
struct DeferredHandleCloseFailures {
    failures: Vec<WindowsApiFailure>,
    omitted_count: usize,
}

#[cfg(windows)]
thread_local! {
    static DEFERRED_HANDLE_CLOSE_FAILURES: RefCell<DeferredHandleCloseFailures> =
        RefCell::new(DeferredHandleCloseFailures::default());
}

#[cfg(windows)]
pub(in crate::run::implementation) fn record_handle_close_failure(code: u32) {
    DEFERRED_HANDLE_CLOSE_FAILURES.with(|failures| {
        let mut failures = failures.borrow_mut();
        if failures.failures.len() < MAX_DEFERRED_HANDLE_CLOSE_DETAILS {
            failures
                .failures
                .push(WindowsApiFailure::new("CloseHandle", code));
        } else {
            failures.omitted_count = failures.omitted_count.saturating_add(1);
        }
    });
}

#[cfg(windows)]
pub(in crate::run::implementation) fn take_deferred_handle_close_error() -> Option<anyhow::Error> {
    let failures =
        DEFERRED_HANDLE_CLOSE_FAILURES.with(|failures| std::mem::take(&mut *failures.borrow_mut()));
    let has_failures = !failures.failures.is_empty() || failures.omitted_count > 0;
    has_failures.then(|| {
        anyhow::Error::new(WindowsJobError::handle_close_failures(
            failures.failures,
            failures.omitted_count,
        ))
    })
}

#[cfg(all(test, windows))]
#[path = "test/mod.rs"]
mod test;
