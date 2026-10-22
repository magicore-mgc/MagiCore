# CAS write / Ghi CAS

This owner preserves the public `cas::write` API while separating write/hash operations, temp-name generation, strict temp-name parsing, parent-directory sync, permissions, and shared local I/O mapping. File creation, cleanup, rename, and store policy remain owned by their current callers.

Owner này giữ nguyên public API `cas::write` và tách thao tác ghi/hash, sinh tên temp, parse tên temp nghiêm ngặt, sync thư mục cha, permission và ánh xạ I/O cục bộ dùng chung. Tạo file, cleanup, rename và policy store vẫn thuộc các caller hiện tại.
