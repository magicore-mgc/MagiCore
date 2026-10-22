# Generation Token Migration Needed

Owns the decision to rebuild a legacy CAS claim table when the generation key or its composite `cas_generations` foreign key is missing or malformed.

## Trách nhiệm

Quản lý quyết định rebuild bảng CAS claim cũ khi thiếu khóa generation hoặc foreign key ghép tới `cas_generations`.

Checks `PRAGMA table_info` and `PRAGMA foreign_key_list`; both key shape and the `(project_root, generation) ON DELETE CASCADE` relation must match before migration can be skipped.

Kiểm tra `PRAGMA table_info` và `PRAGMA foreign_key_list`; phải khớp cả hình dạng khóa lẫn quan hệ `(project_root, generation) ON DELETE CASCADE` mới được bỏ qua migration.
