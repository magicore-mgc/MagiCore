# Database restore

Writes `store.db` through the shared regular-file no-follow staging owner. The destination parent is created and checked component by component; an existing symlink/non-regular destination is refused by the shared file-copy owner. Publication replaces the target atomically when the platform helper supports it.

# Khôi phục database

Ghi `store.db` qua owner staging no-follow cho file thường dùng chung. Parent đích được tạo và kiểm tra từng component; destination symlink/không phải file thường bị owner copy từ chối. Publication thay thế target atomic khi helper nền tảng hỗ trợ.
