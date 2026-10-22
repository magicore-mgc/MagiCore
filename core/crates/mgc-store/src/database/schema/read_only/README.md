# Read-only database open

This additive `Database::open_read_only` API opens an existing regular file with SQLite read-only and no-follow flags. It performs only the schema-version read; unsupported versions fail instead of invoking the migration sequence in `Database::open`. It does not create a missing DB or execute DB writes. SQLite WAL-mode reads may still require or create `-wal`/`-shm` sidecars depending on their existing state and directory permissions; this API therefore does not promise zero filesystem metadata changes. It does not close path races for ancestor components.

## Files

- `read_only.rs`: path checks, read-only connection and schema compatibility gate.
- `error.rs`: local typed errors retaining I/O and SQLite causes.
- `test/`: nearest tests for missing DB, old schema, denied mutation and final-component symlink.
- `mod.rs`: narrow owner wiring.

# Mở database chỉ đọc

API bổ sung `Database::open_read_only` mở file thường đã có bằng cờ SQLite chỉ đọc và không-follow. Nó chỉ đọc schema version; schema không hỗ trợ sẽ bị từ chối thay vì chạy migration của `Database::open`. API không tạo DB thiếu và không thực thi lệnh ghi DB. Đọc DB ở WAL mode vẫn có thể cần hoặc tạo sidecar `-wal`/`-shm` tùy trạng thái sẵn có và quyền thư mục; vì vậy API không cam kết filesystem hoàn toàn không đổi metadata. Nó cũng chưa đóng race ở component thư mục cha.
