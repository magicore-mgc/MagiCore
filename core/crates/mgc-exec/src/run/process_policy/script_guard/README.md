# Script guard

This owner reads the first shebang of an existing script file and delegates interpreter classification to the shared process matcher. It preserves the existing permissive result for non-files, unreadable files, and scripts without a shebang. Failures use the shared `ProcessPolicyError` and keep the old diagnostic.

## Guard script

Owner đọc shebang đầu tiên của file script có sẵn rồi dùng matcher process chung để phân loại interpreter. Owner giữ nguyên kết quả permissive với đường dẫn không phải file, file không đọc được và file không có shebang. Lỗi dùng `ProcessPolicyError` chung và giữ diagnostic cũ.
