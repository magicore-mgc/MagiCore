# Existing Database Open

This additive API opens an already-existing regular SQLite database for normal schema checks/migrations and writes, while omitting `SQLITE_OPEN_CREATE` and enabling `SQLITE_OPEN_NOFOLLOW`. Missing files and final-component symlinks therefore fail at SQLite open time. The existing `Database::open` API still uses rusqlite's default flags and retains its historical create-if-missing behavior.

API bổ sung này mở SQLite file thường đã tồn tại để kiểm tra/migration schema và ghi như bình thường, nhưng bỏ `SQLITE_OPEN_CREATE` và bật `SQLITE_OPEN_NOFOLLOW`. File thiếu và symlink ở component cuối sẽ lỗi ngay lúc SQLite mở. API `Database::open` hiện tại vẫn giữ default flags của rusqlite và hành vi lịch sử tự tạo file thiếu.

`open_existing.rs` owns the narrow public method, `error.rs` preserves its failure source, and `test/` checks no-create, existing-file migration and symlink rejection. Writable `Database::open` and `Database::open_existing` share schema initialization; `Database::open_read_only` remains separate and never invokes that migration sequence.

`open_existing.rs` sở hữu method public hẹp, `error.rs` giữ nguồn lỗi, `test/` kiểm tra không tạo file, migration trên file có sẵn và từ chối symlink. `Database::open` và `Database::open_existing` dùng chung logic schema cho đường ghi; `Database::open_read_only` tách riêng và không gọi chuỗi migration đó.
