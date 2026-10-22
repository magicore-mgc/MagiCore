# App dependency install

This owner implements native App install routing. App project identity/language, React Native gating, manifest hints and shared typed errors belong to `core/app/`; `error.rs` owns install-only package-argument refusal. This folder keeps install-specific Flutter/App orchestration and calls the shared native dependency pipeline. It does not spawn Flutter or another package manager. Existing `core::install::app::*` paths are retained by re-export. Tests are NOT_RUN.

# Cài dependency App

Owner này triển khai định tuyến install App native. Identity/language, gate React Native, manifest hint và lỗi typed dùng chung thuộc `core/app/`; `error.rs` sở hữu lỗi từ chối package arguments riêng install. Folder này giữ orchestration Flutter/App install và gọi pipeline dependency native dùng chung. Không spawn Flutter hay package manager khác. Đường dẫn `core::install::app::*` được giữ bằng re-export. Test NOT_RUN.
