//! Deferred native handle-close failure owner facade.
//! Facade owner lưu lỗi đóng native handle để báo lại sau.

#[path = "deferred_handle_close.rs"]
mod implementation;

// `owned_handle` records a close failure through this facade, so the recorder
// must be reachable from the sibling module — it was defined but never
// forwarded, which only the Windows build noticed.
// (`owned_handle` ghi lỗi đóng qua facade này, nên hàm ghi phải tới được từ
// module anh em — nó có định nghĩa nhưng chưa bao giờ được chuyển tiếp, chỉ
// build Windows mới thấy.)
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::{
    record_handle_close_failure, take_deferred_handle_close_error,
};
