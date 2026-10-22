# Storage errors

This owner retains the original `std::io::Error` for store inspection, temporary write-probe cleanup, disk-space lookup and explicit store initialization. The caller can turn expected inspection failures into doctor diagnostics without losing the OS error source.

# Lỗi storage

Owner này giữ nguyên `std::io::Error` khi kiểm tra store, dọn file thử quyền ghi, đọc dung lượng đĩa và khởi tạo store theo yêu cầu. Caller có thể biến lỗi kiểm tra dự kiến thành diagnostic mà vẫn giữ source lỗi của hệ điều hành.
