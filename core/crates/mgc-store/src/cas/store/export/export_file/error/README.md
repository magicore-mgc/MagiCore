# Export Source Error owner

Owns typed failures while opening a CAS export source and preserves the existing not-found and non-regular-file diagnostics.

Quản lý lỗi có kiểu khi mở nguồn export CAS và giữ nguyên chẩn đoán file không tồn tại hoặc không phải file thường.

Tests live in `test/`; the shared platform opener rejects final symlinks and non-regular files before returning a handle.

Test nằm trong `test/`; opener dùng chung của platform từ chối symlink cuối và đối tượng không phải file thường trước khi trả handle.
