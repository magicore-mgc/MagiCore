# Escape metadata dot segments / Escape dot-segment metadata

This owner checks that raw `.` and `..` are converted to ordinary single path segments before joining and that a `..` metadata write does not use the lexical parent path. It does not claim to prove containment against a pre-existing symlink inside the cache root.

Owner này kiểm tra `.` và `..` thô được đổi thành segment đơn bình thường trước khi ghép path và thao tác ghi `..` không dùng lexical parent path. Test này không khẳng định chặn symlink đã tồn tại bên trong cache root.
