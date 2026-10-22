# Prune root safety

Validates the project root, `.magicore/cache/web` ancestry, lock directory, existing database file, and project virtual-store root before SQLite or recursive deletion. `error.rs` owns the local typed path/database failures. Missing project store/DB paths return a no-op for prune. Symlinks and non-directory roots fail closed. Checks are path-based and do not eliminate concurrent ancestor replacement.

# An toàn root prune

Xác thực project root, cây `.magicore/cache/web`, thư mục lock, file DB có sẵn và virtual-store theo project trước khi mở SQLite hoặc xóa đệ quy. `error.rs` sở hữu lỗi path/database có type tại chỗ. Store/DB chưa có trả no-op cho prune. Symlink và root sai loại bị chặn. Kiểm tra dựa trên path và chưa loại bỏ race thay ancestor đồng thời.
