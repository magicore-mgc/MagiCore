# Tree copy

Streams regular files through the shared no-follow atomic-file owner, creates real directories first, then rescans and rebases in-tree links. The second pass avoids a memory vector proportional to package-link count and supports Windows link targets that must already exist.

# Copy cây

Stream file thường qua owner file atomic no-follow dùng chung, tạo directory thật trước, rồi quét lại và rebase link nội bộ. Lượt hai tránh vector memory tăng theo số package link và hỗ trợ Windows cần target tồn tại trước lúc tạo link.
