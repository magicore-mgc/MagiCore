# Prune

## Responsibility

Prunes unreferenced packages and renders dry-run/delete results.

## Trách nhiệm

Prune package không còn tham chiếu và hiển thị kết quả dry-run/xóa.

`prune.rs` coordinates one maintenance-lock scope, opens existing state, preflights every unreferenced package, then performs deletions. `root_safety/` rejects unsafe store, DB, lock and virtual-store roots; `candidate_plan/` maps package IDs to the same scoped directory names used by the Web materializer and measures logical bytes before mutation; `error.rs` keeps path/removal failures typed; `test/` covers dry-run, mutation, lock contention and symlink refusal. `mod.rs` only wires this owner.

`prune.rs` điều phối một phạm vi maintenance lock, mở state đã có, kiểm tra trước mọi package không tham chiếu rồi mới xóa. `root_safety/` chặn root store, DB, lock và virtual-store không an toàn; `candidate_plan/` ánh xạ package ID theo đúng tên thư mục scope của Web materializer và đo payload logic trước khi sửa; `error.rs` giữ lỗi path/xóa có type; `test/` kiểm tra dry-run, mutation, tranh chấp lock và từ chối symlink. `mod.rs` chỉ nối owner.

Dry-run does not write SQLite rows or remove package directories. Lock setup can create lock metadata, and reading a SQLite WAL database can create `-wal`/`-shm` sidecars; zero filesystem metadata changes are not promised. Both dry-run and real prune open only existing DB files and reject final symlinks at SQLite open time. Path-based checks do not close hostile concurrent ancestor/path replacement races.

Dry-run không ghi row SQLite hoặc xóa thư mục package. Thiết lập lock có thể tạo metadata lock, và đọc SQLite WAL có thể tạo sidecar `-wal`/`-shm`; không cam kết filesystem hoàn toàn không đổi metadata. Cả dry-run và prune thật chỉ mở DB có sẵn, đồng thời SQLite từ chối symlink component cuối. Kiểm tra theo path chưa chặn race thay component thư mục cha/path đồng thời có chủ đích.

Triển khai: `prune.rs`; `mod.rs` chỉ nối owner này.
