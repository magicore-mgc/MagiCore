//! Turn a raw toolchain failure into the exact command that fixes it.
//! Biến lỗi toolchain thô thành đúng lệnh khắc phục.
//!
//! A build that fails because a Rust target or a Python build backend is missing
//! used to surface only Cargo's or Python's own text, which names neither the
//! missing piece nor the fix. This owner classifies that output once and returns
//! a remedy line the caller appends to its own error, so every build lane gives
//! the same actionable sentence instead of each one inventing its own.
//! Build hỏng vì thiếu Rust target hay Python build backend từng chỉ hiện text
//! của Cargo/Python — không nói thiếu gì, không nói sửa sao. Owner này phân loại
//! output một lần và trả dòng khắc phục để caller thêm vào lỗi của mình, nên mọi
//! lane build cùng một câu hướng dẫn thay vì mỗi nơi tự đặt một câu.

/// Markers Cargo prints when the selected target's std is not installed.
/// Dấu hiệu Cargo in ra khi std của target đang chọn chưa được cài.
const RUST_TARGET_MISSING_MARKERS: &[&str] = &[
    "can't find crate for `core`",
    "can't find crate for `std`",
    "target may not be installed",
    "is not installed for the target",
];

/// Markers the active Python interpreter prints when `build` is absent.
/// Dấu hiệu interpreter Python đang chạy in ra khi thiếu `build`.
const PYTHON_BUILD_BACKEND_MISSING_MARKERS: &[&str] = &[
    "No module named build",
    "No module named 'build'",
    "No module named build.__main__",
    "No module named 'build.__main__'",
];

/// Context a build lane already knows about the failing command.
/// Ngữ cảnh mà lane build đã biết về lệnh đang hỏng.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ToolchainContext<'a> {
    /// Rust target triple passed as `cargo build --target <triple>`.
    /// Triple target Rust truyền qua `cargo build --target <triple>`.
    pub rust_target: Option<&'a str>,
    /// Python interpreter that ran `-m build` (`python3`, `python`, ...).
    /// Interpreter Python đã chạy `-m build` (`python3`, `python`, ...).
    pub python: Option<&'a str>,
}

/// Return the remedy for a toolchain failure, or `None` when the output is
/// something else (a compile error must not be dressed up as a setup problem).
/// Trả lệnh khắc phục cho lỗi toolchain, hoặc `None` khi output là lỗi khác
/// (lỗi biên dịch không được gắn nhãn "thiếu cài đặt").
pub(crate) fn toolchain_remedy(output_tail: &str, context: ToolchainContext<'_>) -> Option<String> {
    if RUST_TARGET_MISSING_MARKERS
        .iter()
        .any(|marker| output_tail.contains(marker))
    {
        let triple = context
            .rust_target
            .filter(|target| !target.is_empty())
            .unwrap_or("<target-triple>");
        return Some(format!(
            "the Rust target '{triple}' is not installed — run `rustup target add {triple}` and retry"
        ));
    }
    if PYTHON_BUILD_BACKEND_MISSING_MARKERS
        .iter()
        .any(|marker| output_tail.contains(marker))
    {
        let python = context
            .python
            .filter(|interpreter| !interpreter.is_empty())
            .unwrap_or("python");
        return Some(format!(
            "the Python build backend is not installed — run `{python} -m pip install build` and retry"
        ));
    }
    None
}

/// Append the remedy to an existing message, when one applies.
/// Thêm lệnh khắc phục vào message sẵn có, khi có áp dụng.
pub(crate) fn with_remedy(
    message: String,
    output_tail: &str,
    context: ToolchainContext<'_>,
) -> String {
    match toolchain_remedy(output_tail, context) {
        Some(remedy) => format!("{message} — {remedy}"),
        None => message,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
