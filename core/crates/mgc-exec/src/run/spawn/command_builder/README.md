# Spawn command builder

This owner chooses the platform command representation, preserves caller arguments and working directory, and applies the requested stdio mode. It does not spawn. It introduces no local data type or command-specific error: the fallible Windows `.cmd`/`.bat` path propagates the typed failures owned by the adjacent `windows_command/` module, avoiding a duplicate wrapper and changed error chain.

Owner này chọn cấu trúc lệnh theo nền tảng, giữ arguments và thư mục làm việc caller, đồng thời cấu hình chế độ stdio. Owner không spawn. Không có data type hay lỗi riêng mới: nhánh Windows `.cmd`/`.bat` truyền nguyên lỗi typed do module `windows_command/` sở hữu, tránh bọc trùng và đổi error chain.
