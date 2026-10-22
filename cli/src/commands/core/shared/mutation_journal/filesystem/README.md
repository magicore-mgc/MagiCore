# Journal filesystem primitives / Primitive filesystem cho journal

Owns symlink/reparse checks, no-follow reads, durable temp-file writes, atomic rename, and directory sync. Other owners use these primitives and do not implement alternate write paths.

Sở hữu kiểm tra symlink/reparse, đọc no-follow, ghi file tạm bền vững, rename nguyên tử và fsync thư mục. Owner khác dùng primitive này, không tự tạo đường ghi thay thế.
