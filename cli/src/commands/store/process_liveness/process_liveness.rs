//! Provides the platform-specific zero-signal process probe.
//! Cung cấp phép dò tiến trình signal 0 theo hệ điều hành.

/// PID liveness probe (Gate 11-B lease GC): zero-signal `kill(pid, 0)`
/// — POSIX semantics, no signal delivered, only existence/permission
/// checked. Returns None where the probe is unavailable (non-unix) —
/// the caller then falls back to the grace window alone (conservative).
/// (Thăm dò pid còn sống: `kill(pid, 0)` zero-signal — semantics POSIX,
/// không gửi tín hiệu, chỉ kiểm tra tồn tại/quyền. Trả None nơi thăm dò
/// không khả dụng (non-unix) — caller rơi về cửa sổ xử lý thuần (bảo
/// toàn).)
/// SAFETY PROOF: `kill(pid, 0)` is the documented POSIX liveness probe
/// — signal 0 delivers NOTHING to the target, the kernel only checks
/// process existence and permission. No memory is touched, no
/// invariants to uphold; the single argument is a plain integer.
/// (Giải trình an toàn: `kill(pid, 0)` là thăm dò tồn tại POSIX có tài
/// liệu — signal 0 KHÔNG gửi gì tới đích, kernel chỉ kiểm tra process
/// tồn tại và quyền. Không chạm bộ nhớ, không invariant phải giữ; đối
/// số duy nhất là số nguyên thuần.)
// SAFETY: kill(pid, 0) only probes existence/permission; it carries no pointer,
// touches no process memory, and its only argument is a plain integer.
// AN TOÀN: kill(pid, 0) chỉ kiểm tồn tại/quyền; không con trỏ, không chạm bộ
// nhớ process, đối số duy nhất là số nguyên.
#[cfg(unix)]
#[allow(unsafe_code)]
pub(in crate::commands::store) fn pid_alive_on_this_os(pid: u32) -> Option<bool> {
    // SAFETY: kill(pid, 0) only probes existence/permission; it carries no
    // pointer, touches no process memory, and its only argument is an integer.
    // AN TOÀN: kill(pid, 0) chỉ kiểm tồn tại/quyền; không con trỏ, không chạm
    // bộ nhớ process, đối số duy nhất là số nguyên.
    let ret = unsafe { libc::kill(pid as libc::pid_t, 0) };
    match ret {
        0 => Some(true),
        -1 => {
            // ESRCH = no such process (dead); EPERM = exists but owned by
            // someone else (ALIVE).
            // (ESRCH = không có process (chết); EPERM = có nhưng thuộc
            // user khác (CÒN SỐNG).)
            Some(std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH))
        }
        _ => Some(false),
    }
}

#[cfg(not(unix))]
pub(in crate::commands::store) fn pid_alive_on_this_os(_pid: u32) -> Option<bool> {
    None
}
