# Backup

## Responsibility

Backs up the project Web database and virtual-store tree while holding the same project maintenance lock as install, prune, and restore. Source roots and destination types are checked before copying; regular files use staged no-follow publication, and the tree owner rebases safe in-tree package links. Backup files preserve the existing `store.db` + `vstore/` layout.

## Trách nhiệm

Backup database Web và cây virtual store của project trong lúc giữ cùng maintenance lock với install, prune và restore. Root nguồn cùng loại đích được kiểm tra trước copy; file thường dùng publication staging no-follow, còn tree owner rebase package link nội bộ an toàn. Bố cục backup hiện hành `store.db` + `vstore/` được giữ nguyên.

`backup.rs` coordinates the operation. `source_snapshot/` owns root checks and the project-lock guard; `destination_validation/` owns destination checks and preparation; `payload_copy/` owns copy order and size reporting. `error.rs` is the feature-local typed error boundary, and each behavior owner keeps its test module nearby. `mod.rs` only wires owners.

The destination may be new or an empty real directory. Existing entries are rejected to avoid silently combining an older snapshot with current project data.

`backup.rs` điều phối thao tác. `source_snapshot/` quản lý kiểm tra root và project-lock guard; `destination_validation/` quản lý kiểm tra/chuẩn bị đích; `payload_copy/` quản lý thứ tự copy và báo kích thước. `error.rs` là ranh giới lỗi có type cục bộ của feature; mỗi behavior owner đặt test gần module. `mod.rs` chỉ nối các owner.

Đích backup có thể chưa tồn tại hoặc là thư mục thật đang trống. Nếu đã có entry, lệnh từ chối để không âm thầm trộn snapshot cũ với dữ liệu project hiện tại.
