//! client diagnostic.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

pub(in crate::native::npm_registry) fn client_builder_error(
    label: &str,
    error: reqwest::Error,
) -> String {
    // A builder error's Display is only "builder error". Keep the causal
    // source for runtime diagnostics, but remove any URL before formatting.
    // (Display chỉ là "builder error"; giữ nguyên nhân nhưng gỡ URL trước.)
    format!("{label}: {:?}", error.without_url())
}
