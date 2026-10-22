# AI cache presence

This owner resolves the validated model path and reports whether filesystem metadata exists. Missing paths return `false`; other inspection errors preserve their source. Metadata inspection follows a selected symlink, matching the former `Path::exists()` behavior.

Owner này resolve model path đã validation và kiểm tra metadata filesystem có tồn tại không. Path thiếu trả `false`; lỗi inspect khác giữ source. Metadata inspection theo symlink được chọn, khớp hành vi `Path::exists()` trước đây.

Nearest presence cases are in `test/is_cached.rs`; typed failures are in `error/`.

Test kiểm tra presence gần owner nằm ở `test/is_cached.rs`; lỗi typed ở `error/`.
