//! Gate installer debug traces behind an explicit environment flag.
//! Cổng trace debug của installer sau một cờ môi trường tường minh.
//!
//! These traces were previously printed unconditionally, so every install wrote
//! them to stderr — including `--quiet` runs and piped output. They are
//! diagnostics for developing the installer, not product output, so they must be
//! opt-in.
//! Các trace này trước đây in vô điều kiện nên mọi lần install đều ghi ra stderr
//! — kể cả `--quiet` và khi output bị pipe. Chúng là chẩn đoán khi phát triển
//! installer, không phải output sản phẩm, nên phải opt-in.

/// Environment flag that enables installer debug traces.
/// Cờ môi trường bật trace debug của installer.
pub(crate) const INSTALL_DEBUG_ENV: &str = "MAGICORE_INSTALL_DEBUG";

/// Whether installer debug traces are enabled for this process.
/// Trace debug installer có được bật cho process này không.
pub(crate) fn install_debug_enabled() -> bool {
    std::env::var_os(INSTALL_DEBUG_ENV).is_some_and(|value| {
        let raw = value.to_string_lossy();
        !raw.is_empty() && raw != "0" && !raw.eq_ignore_ascii_case("false")
    })
}

/// Print an installer debug trace only when the flag is set.
/// Chỉ in trace debug installer khi cờ được bật.
pub(crate) fn install_debug(message: std::fmt::Arguments<'_>) {
    if install_debug_enabled() {
        eprintln!("[magicore:debug] {message}");
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
