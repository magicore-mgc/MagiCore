# Atomic lockfile / Ghi lockfile nguyên tử

This folder preserves the public `mgc_lockfile::atomic::*` paths and wires separate owners for failpoints, stale temporary files, lock-protected writes, platform replacement, and v4 writes.

Thư mục này giữ nguyên đường dẫn công khai `mgc_lockfile::atomic::*`, đồng thời nối các owner riêng cho failpoint, file tạm cũ, ghi có project lock, thay file theo hệ điều hành và ghi v4.

`mod.rs` is wiring and explicit re-exports only. Each behavior owner has its own implementation, tests, and contract. Local write errors map back to the existing `LockfileError` variants and diagnostic text at the compatibility boundary.

`mod.rs` chỉ khai báo và export tường minh. Mỗi owner hành vi có implementation, test và contract riêng. Lỗi ghi cục bộ được ánh xạ về `LockfileError` và thông báo hiện tại tại boundary tương thích.

The crash-recovery order, symlink protections, temp naming, stale-owner proof, Windows replacement flags, and public signatures are preserved. Tests are authored separately and have not been executed in this source-first phase.

Thứ tự phục hồi sau crash, chống symlink, cách đặt tên temp, bằng chứng process chết, cờ thay file Windows và chữ ký API được giữ nguyên. Test được tách riêng nhưng chưa chạy trong giai đoạn ưu tiên refactor source.
