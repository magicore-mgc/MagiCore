//! Output-reader and drain implementation export.
//! Export implementation đọc reader và drain output.

#[path = "output_capture.rs"]
mod implementation;

pub(super) use implementation::OutputCapture;
