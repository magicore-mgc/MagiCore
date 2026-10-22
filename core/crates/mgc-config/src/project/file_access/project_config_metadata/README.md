# Project config metadata guard / Kiểm tra metadata cấu hình project

Owns the `mgc.toml` path check used before project identity reads and writes. It accepts only an existing regular file and rejects symlinks and special files before metadata is reused.

Quản lý kiểm tra đường dẫn `mgc.toml` trước khi đọc/ghi identity. Chỉ chấp nhận file thường đã tồn tại, từ chối symlink và file đặc biệt trước khi tái sử dụng metadata.
