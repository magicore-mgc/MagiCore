# Toolchain issue policy

This owner preserves the required-tool warning for missing `git` and `node`. For any discovered tool whose guarded fixed version probe failed, timed out or produced no output (`version == "?"`), it adds `WARN_TOOLCHAIN_VERSION_UNAVAILABLE` and an inspection action without suggesting an unsafe automatic repair. Other inventory rows remain informational when their path and version are available.

# Policy lỗi toolchain

Owner này giữ cảnh báo tool bắt buộc khi thiếu `git` và `node`. Với tool đã tìm được nhưng probe version cố định qua MGC bị lỗi, timeout hoặc không có output (`version == "?"`), owner thêm `WARN_TOOLCHAIN_VERSION_UNAVAILABLE` và action kiểm tra, không gợi ý tự sửa thiếu an toàn. Các inventory row khác chỉ mang tính thông tin khi đã có path và version.
