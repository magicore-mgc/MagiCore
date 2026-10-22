# Atomic project config writer / Bộ ghi config project atomic

Owns staged `mgc.toml` replacement: preserve existing permissions, create a unique same-directory file with restrictive Unix mode, write and sync it, recheck the destination, atomically replace, sync the parent on Unix, and remove the staging file on failure.

Quản lý thay thế `mgc.toml` qua staging: giữ permissions cũ, tạo file duy nhất cùng thư mục với mode Unix hạn chế, ghi và sync, kiểm tra lại đích, atomic replace, sync thư mục cha trên Unix và dọn staging khi lỗi.
