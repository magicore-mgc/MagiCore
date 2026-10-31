//! Shared test support for PyPI cases that touch the process environment.
//! Hỗ trợ test dùng chung cho các case PyPI có thay đổi môi trường process.

/// Serializes tests that rewrite `MGC_PYTHON_VERSION`.
///
/// The variable is process-wide and the harness runs cases in parallel
/// threads, so one lock must be shared by every file that mutates it;
/// per-file locks would still let two files race.
/// (`MGC_PYTHON_VERSION` là env toàn process và harness chạy song song, nên một
/// khóa phải dùng chung cho mọi file thay đổi nó.)
pub(super) fn python_version_env_serial() -> std::sync::MutexGuard<'static, ()> {
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    SERIAL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
