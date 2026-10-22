# Atomic replacement / Thay file atomic

This platform owner publishes a same-filesystem staging file over a destination without a remove-then-rename gap. POSIX uses `rename`; Windows uses replace-existing and can apply write-through through the explicit mode function. The default helper keeps write-through enabled for existing lockfile behavior. `error.rs` preserves staging/destination paths and the original OS error; the compatibility helper maps this typed error back to `std::io::Error`.

Owner nền tảng này publish staging file cùng filesystem đè lên destination mà không có khoảng trống xóa-rồi-rename. POSIX dùng `rename`; Windows dùng cờ replace-existing và nhận write-through qua hàm mode tường minh. Helper mặc định bật write-through để giữ behavior lockfile. `error.rs` giữ path staging/destination cùng OS error gốc; facade tương thích chuyển lỗi typed về `std::io::Error`.

Callers own staging validity, concurrent-writer semantics and parent-directory sync. This operation is path-based; callers must confine destination parent components and own the temp path.

Caller chịu trách nhiệm tính hợp lệ của staging, xử lý writer đồng thời và sync thư mục cha. Thao tác này dùng pathname; caller phải giới hạn parent directory và sở hữu temp path.
