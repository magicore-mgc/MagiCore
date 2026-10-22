# Rust artifact cache / Cache artifact Rust

This owner builds Cargo registry archive paths and reads cached `.crate` files through a no-follow regular-file handle before verifying the locked SHA-256. Missing files remain cache misses; symlinks, reparse points, non-regular files and digest mismatches remain typed failures.

Owner này tạo path archive Cargo và đọc `.crate` qua file handle regular không theo link trước khi xác minh SHA-256 trong lock. File thiếu vẫn là cache miss; symlink, reparse point, file đặc biệt và hash sai tiếp tục trả lỗi typed.
