# Hardware owner tests

`hardware.rs` protects an existing real directory and rejects files or directory symlinks at the target path. `error.rs` checks the unchanged missing-layer diagnostic plus resolver, target-inspection and scaffold source retention. These test sources have not been compiled or run.

# Test owner Hardware

`hardware.rs` giữ nguyên thư mục thật đã có và từ chối file hoặc symlink thư mục tại target. `error.rs` kiểm tra diagnostic thiếu layer không đổi và giữ source của lỗi resolver, kiểm tra target, scaffold. Các source test này chưa được compile hoặc chạy.
