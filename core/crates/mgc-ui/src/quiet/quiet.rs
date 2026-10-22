//! Process-local quiet state — trạng thái quiet trong tiến trình.
//! Atomic ordering and initial state match the previous crate facade.
//! Thứ tự atomic và giá trị khởi đầu giữ nguyên như facade trước.

use std::sync::atomic::{AtomicBool, Ordering};

static QUIET: AtomicBool = AtomicBool::new(false);

#[cfg(test)]
static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Set quiet output mode — bật hoặc tắt chế độ quiet.
pub fn set_quiet(quiet: bool) {
    QUIET.store(quiet, Ordering::Relaxed);
}

/// Read quiet output mode — đọc trạng thái quiet hiện tại.
pub fn is_quiet() -> bool {
    QUIET.load(Ordering::Relaxed)
}

#[cfg(test)]
pub(crate) fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
