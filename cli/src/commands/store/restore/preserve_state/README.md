# Preserve state

Copies only components that exist in the project. Database copies use the shared no-follow staged atomic file owner; virtual-store copies use the validated tree owner, preserving/rebasing valid in-tree symlinks. This stage runs before either restore component is written.

# Bảo toàn state

Chỉ copy thành phần đang tồn tại trong project. Database dùng owner file staging atomic no-follow dùng chung; virtual store dùng owner copy cây đã xác thực, giữ/rebase symlink nội bộ hợp lệ. Bước này chạy trước khi ghi thành phần restore.
